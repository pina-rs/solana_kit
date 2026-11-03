//! Hand-drawn ink from skribble's rough engine.
//!
//! The site's chrome (header, sidebar, code, tables, callouts, cards, and the
//! pager) is rendered by `leptoskit_docs`, so its ink cannot be drawn into the
//! markup. Instead, [`files`] draws a small set of rough shapes into SVG files
//! that `theme/skribble.css` uses as CSS masks: a pseudo-element is painted
//! in a palette colour and the mask cuts the wobbly stroke out of it, so the
//! ink follows the light and dark themes without JavaScript.
//!
//! Every file stretches to its element with `preserveAspectRatio="none"`,
//! while `vector-effect="non-scaling-stroke"` keeps each stroke at the same
//! width at any size. The same seed always draws the same wobble, so the
//! files are byte-stable between builds.

use std::f64::consts::PI;
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::Path;

use skribble_rough::DrawConfig;
use skribble_rough::DrawOptions;
use skribble_rough::Drawable;
use skribble_rough::FillOptions;
use skribble_rough::FillStyle;
use skribble_rough::Filler;
use skribble_rough::Generator;
use skribble_rough::PathElementOptions;
use skribble_rough::Point;
use skribble_rough::RoughSvgPaths;

/// A colour from the site palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    /// The text colour: plum ink on light paper, cream on dark.
    Ink,
    /// The page colour, for knocking shapes out of ink.
    Paper,
    /// The primary accent.
    Coral,
    /// The marker yellow.
    Sun,
    /// Purple, Solana's gradient end.
    Lilac,
    /// Teal, Solana's gradient start.
    Mint,
    /// Blue.
    Sky,
    /// Pink.
    Pink,
}

impl Tone {
    /// The custom property name, without the leading dashes.
    pub fn name(self) -> &'static str {
        match self {
            Self::Ink => "sk-ink",
            Self::Paper => "sk-paper",
            Self::Coral => "sk-coral",
            Self::Sun => "sk-sun",
            Self::Lilac => "sk-lilac",
            Self::Mint => "sk-mint",
            Self::Sky => "sk-sky",
            Self::Pink => "sk-pink",
        }
    }

    /// The colour on light paper, for images that cannot read the page's
    /// custom properties.
    pub fn light(self) -> &'static str {
        match self {
            Self::Ink => "#34283f",
            Self::Paper => "#fffaf0",
            Self::Coral => "#e2674d",
            Self::Sun => "#ffcc4d",
            Self::Lilac => "#9c7bd4",
            Self::Mint => "#3fb3a4",
            Self::Sky => "#4e8fda",
            Self::Pink => "#e57aa0",
        }
    }

    /// The colour on dark paper.
    pub fn dark(self) -> &'static str {
        match self {
            Self::Ink => "#f3ebdf",
            Self::Paper => "#1b1720",
            Self::Coral => "#f08e76",
            Self::Sun => "#f2c14e",
            Self::Lilac => "#b89ce6",
            Self::Mint => "#5cc8b8",
            Self::Sky => "#79aef0",
            Self::Pink => "#f09bbb",
        }
    }
}

/// How a drawing resolves its [`Tone`]s.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Palette {
    /// `var(--sk-…)`, for SVG inside the page, which follows the theme.
    Page,
    /// Fixed light-paper colours, for image files.
    Light,
    /// Fixed dark-paper colours, for image files.
    Dark,
}

impl Palette {
    fn paint(self, tone: Tone) -> String {
        match self {
            Self::Page => format!("var(--{})", tone.name()),
            Self::Light => tone.light().to_owned(),
            Self::Dark => tone.dark().to_owned(),
        }
    }
}

/// How many distinct seeds [`seed_for`] hands out.
const SEED_RANGE: u32 = 100_000;

/// A stable seed derived from `text` (FNV-1a), for ink that should differ
/// between elements but never between builds.
pub fn seed_for(text: &str) -> i64 {
    let mut hash: u32 = 0x811c_9dc5;

    for unit in text.encode_utf16() {
        hash = (hash ^ u32::from(unit)).wrapping_mul(0x0100_0193) & 0x7fff_ffff;
    }

    i64::from(hash % SEED_RANGE + 1)
}

/// A generator whose outline and fill both replay from `seed`.
///
/// The fill gets its own configuration, since drawing advances a
/// configuration's random stream.
pub fn pen(seed: i64, roughness: f64, fill: FillStyle, gap: f64) -> Generator {
    let options = DrawOptions {
        seed,
        roughness,
        ..DrawOptions::default()
    };
    let pattern = FillOptions {
        hachure_gap: gap,
        ..FillOptions::default()
    };

    Generator::new(
        DrawConfig::new(options),
        Filler::new(fill, pattern, DrawConfig::new(options)),
    )
}

/// A nominal canvas that ink is drawn on before it stretches to its element.
///
/// Server rendering cannot measure elements, so a canvas near the element's
/// usual proportions keeps the stretched wobble from looking squashed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Canvas {
    /// Nominal width, in SVG user units.
    pub width: f64,
    /// Nominal height, in SVG user units.
    pub height: f64,
    /// Distance from the canvas edge to the ideal outline, leaving room for
    /// the wobble.
    pub inset: f64,
    /// Distance between sketched fill lines.
    pub gap: f64,
}

impl Canvas {
    /// A box, button, or code block.
    pub const BOX: Self = Self::new(240.0, 120.0, 4.0, 8.0);
    /// A roomy card or callout.
    pub const CARD: Self = Self::new(320.0, 200.0, 4.0, 10.0);
    /// A square for circles and badges.
    pub const SQUARE: Self = Self::new(120.0, 120.0, 4.0, 8.0);
    /// A horizontal rule.
    pub const RULE: Self = Self::new(600.0, 12.0, 2.0, 8.0);
    /// A vertical rule.
    pub const EDGE: Self = Self::new(12.0, 600.0, 2.0, 8.0);
    /// A word, for underlines and marker highlights.
    pub const WORD: Self = Self::new(200.0, 40.0, 3.0, 5.0);

    /// Creates a canvas.
    pub const fn new(width: f64, height: f64, inset: f64, gap: f64) -> Self {
        Self {
            width,
            height,
            inset,
            gap,
        }
    }

    fn view_box(self) -> String {
        format!("0 0 {} {}", number(self.width), number(self.height))
    }
}

/// The shapes the stretched ink can draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// A rough rectangle.
    Rectangle,
    /// A rough ellipse touching the middle of each side.
    Ellipse,
    /// A rough line across the middle.
    Rule,
    /// A rough line down the middle.
    Edge,
    /// A loose underline swept below a word.
    Underline,
    /// A loop drawn around a word, overshooting where it closes.
    Loop,
}

/// Heights of the underline's four points, as fractions of the canvas.
const UNDERLINE_HEIGHTS: [f64; 4] = [0.72, 0.62, 0.7, 0.58];
/// Where the underline's two middle points sit across the canvas.
const UNDERLINE_SPANS: [f64; 2] = [0.35, 0.7];

/// How many segments the loop is drawn with.
const LOOP_STEPS: u8 = 14;
/// Where the loop starts, in half turns: just above the right-hand side, so
/// its overshoot lands beside the word rather than over the line above.
const LOOP_START: f64 = -0.1;
/// How far the loop travels, in half turns: a full turn and a little more,
/// so it overshoots where it closes.
const LOOP_SWEEP: f64 = 2.15;
/// How far each step wanders outwards, cycling every three steps.
const LOOP_WANDER: f64 = 0.05;
/// Shrinks the loop's width so its widest wander stays on the canvas.
const LOOP_WIDTH_SHRINK: f64 = 1.1;
/// Shrinks the loop's height so its overshoot stays on the canvas.
const LOOP_HEIGHT_SCALE: f64 = 0.8;

/// Draws `shape` inset inside `canvas`.
pub fn draw(
    shape: Shape,
    canvas: Canvas,
    seed: i64,
    roughness: f64,
    fill: FillStyle,
) -> RoughSvgPaths {
    let mut generator = pen(seed, roughness, fill, canvas.gap);
    let Canvas {
        width,
        height,
        inset,
        ..
    } = canvas;

    let drawable = match shape {
        Shape::Rectangle => {
            generator.rectangle(inset, inset, width - inset * 2.0, height - inset * 2.0)
        }
        Shape::Ellipse => generator.ellipse(
            width / 2.0,
            height / 2.0,
            width - inset * 2.0,
            height - inset * 2.0,
        ),
        Shape::Rule => generator.line(inset, height / 2.0, width - inset, height / 2.0),
        Shape::Edge => generator.line(width / 2.0, inset, width / 2.0, height - inset),
        Shape::Underline => generator.curve(&underline_points(canvas)),
        Shape::Loop => generator.curve(&loop_points(canvas)),
    };

    RoughSvgPaths::from_drawable(&drawable)
}

/// The curve a loose underline follows, from one end of the word to the
/// other.
fn underline_points(canvas: Canvas) -> [Point; 4] {
    let Canvas {
        width,
        height,
        inset,
        ..
    } = canvas;
    let [first, second, third, last] = UNDERLINE_HEIGHTS.map(|fraction| height * fraction);
    let [near, far] = UNDERLINE_SPANS.map(|fraction| width * fraction);

    [
        Point::new(inset, first),
        Point::new(near, second),
        Point::new(far, third),
        Point::new(width - inset, last),
    ]
}

/// The points a loop around a word passes through.
fn loop_points(canvas: Canvas) -> Vec<Point> {
    let Canvas {
        width,
        height,
        inset,
        ..
    } = canvas;
    let steps = f64::from(LOOP_STEPS);

    (0..=LOOP_STEPS)
        .map(|step| {
            let angle = PI * LOOP_START + f64::from(step) * (PI * LOOP_SWEEP / steps);
            let wander = 1.0 + LOOP_WANDER * f64::from(step % 3);

            Point::new(
                width / 2.0 + (width / 2.0 - inset) / LOOP_WIDTH_SHRINK * angle.cos() * wander,
                height / 2.0 + (height / 2.0 - inset) * LOOP_HEIGHT_SCALE * angle.sin(),
            )
        })
        .collect()
}

/// The width of every outline, in pixels; strokes keep it at any size.
const OUTLINE_WIDTH: f64 = 2.0;
/// The width of sketched fill lines, in pixels.
const HATCH_WIDTH: f64 = 1.5;

/// A standalone SVG file for a CSS mask: every group drawn in black.
pub fn mask(paths: &RoughSvgPaths, canvas: Canvas) -> String {
    let elements = paths.to_path_elements(&PathElementOptions {
        stroke: "#000",
        fill: "#000",
        stroke_width: OUTLINE_WIDTH,
        sketch_width: HATCH_WIDTH,
        non_scaling_stroke: true,
    });

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{}" preserveAspectRatio="none">{elements}</svg>"#,
        canvas.view_box()
    )
}

/// A standalone SVG file stroked in a fixed `color`, for backgrounds that
/// cannot be masks, such as frames on scrolling elements.
pub fn frame(paths: &RoughSvgPaths, canvas: Canvas, color: &str) -> String {
    let elements = paths.to_path_elements(&PathElementOptions {
        stroke: color,
        fill: color,
        stroke_width: OUTLINE_WIDTH,
        sketch_width: HATCH_WIDTH,
        non_scaling_stroke: true,
    });

    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{}" preserveAspectRatio="none">{elements}</svg>"#,
        canvas.view_box()
    )
}

/// Only the sketched fill of `paths`, as a mask file.
pub fn hatch_mask(paths: &RoughSvgPaths, canvas: Canvas) -> String {
    mask(
        &RoughSvgPaths {
            sketch: paths.sketch.clone(),
            ..RoughSvgPaths::default()
        },
        canvas,
    )
}

/// A decorative `<svg>` that stretches over its positioned parent, stroked
/// in `currentColor` and filled with `--sk-fill`.
pub fn overlay(paths: &RoughSvgPaths, canvas: Canvas) -> String {
    let mut svg = format!(
        r#"<svg class="sk-ink" viewBox="{}" preserveAspectRatio="none" aria-hidden="true" focusable="false">"#,
        canvas.view_box()
    );

    if !paths.solid.is_empty() {
        let _ = write!(
            svg,
            r#"<path d="{}" style="fill:var(--sk-fill,currentColor);stroke:none"/>"#,
            paths.solid
        );
    }

    let strokes = [
        (
            &paths.sketch,
            "sk-ink-hatch",
            "var(--sk-fill,currentColor)",
            HATCH_WIDTH,
        ),
        (
            &paths.outline,
            "sk-ink-outline",
            "currentColor",
            OUTLINE_WIDTH,
        ),
    ];

    for (data, class, paint, width) in strokes {
        if data.is_empty() {
            continue;
        }

        let _ = write!(
            svg,
            r#"<path class="{class}" d="{data}" style="fill:none;stroke:{paint};stroke-width:{}px;stroke-linecap:round;stroke-linejoin:round" vector-effect="non-scaling-stroke"/>"#,
            number(width)
        );
    }

    svg.push_str("</svg>");
    svg
}

/// One drawable in a [`doodle`], with the tones it is painted in.
#[derive(Debug, Clone)]
pub struct Mark {
    /// The rough shape.
    pub drawable: Drawable,
    /// The outline tone.
    pub stroke: Tone,
    /// The fill tone for solid and hatched fills; defaults to `stroke`.
    pub fill: Option<Tone>,
    /// The outline width, before scaling.
    pub width: f64,
}

impl Mark {
    /// A mark outlined in `stroke` at the default width.
    pub fn new(drawable: Drawable, stroke: Tone) -> Self {
        Self {
            drawable,
            stroke,
            fill: None,
            width: OUTLINE_WIDTH,
        }
    }

    /// Fills the mark with `fill`.
    #[must_use]
    pub fn fill(mut self, fill: Tone) -> Self {
        self.fill = Some(fill);
        self
    }

    /// Sets the outline width.
    #[must_use]
    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }
}

/// A multi-coloured rough drawing at its intrinsic size.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Doodle<'a> {
    /// The drawing's intrinsic width.
    pub width: f64,
    /// The drawing's intrinsic height.
    pub height: f64,
    /// How the drawing resolves its tones.
    pub palette: Palette,
    /// A CSS class for the `<svg>`.
    pub class: Option<&'a str>,
    /// An accessible name. Without one the drawing is hidden from assistive
    /// technology.
    pub label: Option<&'a str>,
}

impl<'a> Doodle<'a> {
    /// A decorative `width` by `height` drawing in `palette`.
    pub fn new(width: f64, height: f64, palette: Palette) -> Self {
        Self {
            width,
            height,
            palette,
            class: None,
            label: None,
        }
    }

    /// Adds a CSS class to the `<svg>`.
    #[must_use]
    pub fn class(mut self, class: &'a str) -> Self {
        self.class = Some(class);
        self
    }

    /// Names the drawing for assistive technology.
    #[must_use]
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Renders `marks`, back to front, as an `<svg>`.
    pub fn render(&self, marks: &[Mark]) -> String {
        let mut svg = self.open();

        for mark in marks {
            self.paint(&mut svg, mark);
        }

        svg.push_str("</svg>");
        svg
    }

    /// The opening `<svg …>` tag.
    fn open(&self) -> String {
        let mut svg = String::from("<svg");

        if self.palette != Palette::Page {
            svg.push_str(r#" xmlns="http://www.w3.org/2000/svg""#);
        }

        if let Some(class) = self.class {
            let _ = write!(svg, r#" class="{class}""#);
        }

        let _ = write!(
            svg,
            r#" viewBox="0 0 {w} {h}" width="{w}" height="{h}" focusable="false""#,
            w = number(self.width),
            h = number(self.height),
        );

        match self.label {
            Some(label) => {
                let _ = write!(svg, r#" role="img" aria-label="{label}""#);
            }
            None => svg.push_str(r#" aria-hidden="true""#),
        }

        svg.push('>');
        svg
    }

    /// Appends one mark's solid fill, sketched fill, and outline.
    fn paint(&self, svg: &mut String, mark: &Mark) {
        let paths = RoughSvgPaths::from_drawable(&mark.drawable);
        let fill = self.palette.paint(mark.fill.unwrap_or(mark.stroke));
        let stroke = self.palette.paint(mark.stroke);

        if !paths.solid.is_empty() {
            let _ = write!(
                svg,
                r#"<path d="{}" style="fill:{fill};stroke:none"/>"#,
                paths.solid
            );
        }

        if !paths.sketch.is_empty() {
            let _ = write!(
                svg,
                r#"<path d="{}" style="fill:none;stroke:{fill};stroke-width:1.4;stroke-linecap:round"/>"#,
                paths.sketch
            );
        }

        if !paths.outline.is_empty() {
            let _ = write!(
                svg,
                r#"<path d="{}" style="fill:none;stroke:{stroke};stroke-width:{};stroke-linecap:round;stroke-linejoin:round"/>"#,
                paths.outline,
                number(mark.width)
            );
        }
    }
}

/// The seed the site draws its mark with.
pub const BRAND_SEED: i64 = 11;
/// The mark's intrinsic width.
pub const BRAND_WIDTH: f64 = 42.0;
/// The mark's intrinsic height.
pub const BRAND_HEIGHT: f64 = 45.0;

/// Solana's top bar, a parallelogram leaning right.
const MINT_BAR: [(f64, f64); 4] = [(13.0, 9.0), (35.0, 9.0), (29.0, 16.0), (7.0, 16.0)];
/// Solana's middle bar, leaning left.
const SKY_BAR: [(f64, f64); 4] = [(7.0, 19.0), (29.0, 19.0), (35.0, 26.0), (13.0, 26.0)];
/// Solana's bottom bar, leaning right.
const LILAC_BAR: [(f64, f64); 4] = [(13.0, 29.0), (35.0, 29.0), (29.0, 36.0), (7.0, 36.0)];
/// The three bars with the tones they are filled in.
const BRAND_BARS: [([(f64, f64); 4], Tone); 3] = [
    (MINT_BAR, Tone::Mint),
    (SKY_BAR, Tone::Sky),
    (LILAC_BAR, Tone::Lilac),
];
/// The left bracket; the right one is its mirror image.
const BRAND_BRACKET: [(f64, f64); 4] = [(5.0, 2.0), (1.5, 2.5), (1.5, 42.5), (5.0, 43.0)];
/// How far the bars wobble.
const BAR_ROUGHNESS: f64 = 0.6;
/// The bars' outline width.
const BAR_OUTLINE: f64 = 1.6;
/// How far the brackets wobble.
const BRACKET_ROUGHNESS: f64 = 0.5;
/// The brackets' stroke width.
const BRACKET_WIDTH: f64 = 2.4;
/// The brackets' seeds start after the bars'.
const BRACKET_SEED_OFFSET: i64 = 5;
/// The hachure gap the mark's pens are built with; the mark has no
/// hachure, so it only keeps the random streams stable.
const BRAND_GAP: f64 = 5.0;

/// The solana_kit mark, drawn by hand on a [`BRAND_WIDTH`] by
/// [`BRAND_HEIGHT`] canvas: Solana's three bars in mint, sky, and lilac, held
/// in a pair of square brackets.
pub fn brand_mark(seed: i64) -> Vec<Mark> {
    let mut marks = Vec::new();

    for (index, (points, tone)) in (0_i64..).zip(BRAND_BARS) {
        let points = points.map(|(x, y)| Point::new(x, y));
        let drawable =
            pen(seed + index, BAR_ROUGHNESS, FillStyle::Solid, BRAND_GAP).polygon(&points);
        marks.push(Mark::new(drawable, Tone::Ink).fill(tone).width(BAR_OUTLINE));
    }

    for (index, left) in (0_i64..).zip([true, false]) {
        let points = BRAND_BRACKET.map(|(x, y)| {
            let x = if left { x } else { BRAND_WIDTH - x };
            Point::new(x, y)
        });
        let seed = seed + BRACKET_SEED_OFFSET + index;
        let drawable =
            pen(seed, BRACKET_ROUGHNESS, FillStyle::None, BRAND_GAP).linear_path(&points, false);
        marks.push(Mark::new(drawable, Tone::Ink).width(BRACKET_WIDTH));
    }

    marks
}

/// How many seeded variants of each mask shape the stylesheet cycles through.
const VARIANTS: i64 = 4;
/// The distance between one variant's seed and the next.
const VARIANT_SEED_STEP: i64 = 11;

/// How far boxes and cards wobble; large frames need more to show it.
const FRAME_MASK_ROUGHNESS: f64 = 1.8;
/// How far rules and edges wobble.
const RULE_ROUGHNESS: f64 = 1.4;
/// How far underlines wobble.
const UNDERLINE_ROUGHNESS: f64 = 1.2;

/// Every outline mask, by file name, with its canvas and roughness.
const MASKS: [(&str, Shape, Canvas, f64); 7] = [
    ("box", Shape::Rectangle, Canvas::BOX, FRAME_MASK_ROUGHNESS),
    ("card", Shape::Rectangle, Canvas::CARD, FRAME_MASK_ROUGHNESS),
    ("circle", Shape::Ellipse, Canvas::SQUARE, 1.0),
    ("rule", Shape::Rule, Canvas::RULE, RULE_ROUGHNESS),
    ("edge", Shape::Edge, Canvas::EDGE, RULE_ROUGHNESS),
    (
        "underline",
        Shape::Underline,
        Canvas::WORD,
        UNDERLINE_ROUGHNESS,
    ),
    ("loop", Shape::Loop, Canvas::WORD, 1.0),
];
/// How far the hatched fill's border wobbles.
const HATCH_ROUGHNESS: f64 = 1.0;
/// Frames are drawn from their own seeds, so they differ from the masks.
const FRAME_SEED_OFFSET: i64 = 3;
/// How far a frame wobbles.
const FRAME_ROUGHNESS: f64 = 1.5;

/// Every file in `theme/ink`, by name: the masks the stylesheet uses and the
/// brand mark on light and dark paper.
pub fn files() -> Vec<(String, String)> {
    let mut files = Vec::new();

    for variant in 1..=VARIANTS {
        variant_files(variant, &mut files);
    }

    let mark = brand_mark(BRAND_SEED);

    for (name, palette) in [("light", Palette::Light), ("dark", Palette::Dark)] {
        let svg = Doodle::new(BRAND_WIDTH, BRAND_HEIGHT, palette)
            .label("solana_kit")
            .render(&mark);
        files.push((format!("mark-{name}.svg"), svg));
    }

    files
}

/// One seeded variant of every mask, the hatch, and the light and dark
/// frames.
fn variant_files(variant: i64, files: &mut Vec<(String, String)>) {
    let seed = variant * VARIANT_SEED_STEP;

    for (name, shape, canvas, roughness) in MASKS {
        let paths = draw(shape, canvas, seed, roughness, FillStyle::None);
        files.push((format!("{name}-{variant}.svg"), mask(&paths, canvas)));
    }

    let hatch = draw(
        Shape::Rectangle,
        Canvas::BOX,
        seed,
        HATCH_ROUGHNESS,
        FillStyle::Hachure,
    );
    files.push((
        format!("hatch-{variant}.svg"),
        hatch_mask(&hatch, Canvas::BOX),
    ));

    let outline = draw(
        Shape::Rectangle,
        Canvas::CARD,
        seed + FRAME_SEED_OFFSET,
        FRAME_ROUGHNESS,
        FillStyle::None,
    );

    for (name, tone) in [("light", Tone::Ink.light()), ("dark", Tone::Ink.dark())] {
        files.push((
            format!("frame-{variant}-{name}.svg"),
            frame(&outline, Canvas::CARD, tone),
        ));
    }
}

/// Writes [`files`] into `dir` and returns how many there are.
///
/// Files that are already current are left alone, so the development
/// server's watcher does not see a change on every start.
pub fn write(dir: &Path) -> io::Result<usize> {
    fs::create_dir_all(dir)?;

    let files = files();

    for (name, svg) in &files {
        let path = dir.join(name);

        if fs::read_to_string(&path).ok().as_deref() != Some(svg.as_str()) {
            fs::write(path, svg)?;
        }
    }

    Ok(files.len())
}

fn number(value: f64) -> String {
    skribble_rough::format_number(value, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeds_are_stable() {
        assert_eq!(seed_for("solana"), seed_for("solana"));
        assert_ne!(seed_for("solana"), seed_for("kit"));
        assert!((1..=100_000).contains(&seed_for("")));
    }

    #[test]
    fn files_are_deterministic() {
        assert_eq!(files(), files());
    }

    #[test]
    fn masks_keep_strokes_at_their_width() {
        for (name, svg) in files() {
            if name.starts_with("mark-") {
                continue;
            }

            assert!(svg.contains(r#"preserveAspectRatio="none""#), "{name}");
            assert!(svg.contains("non-scaling-stroke"), "{name}");
            assert!(svg.contains("<path"), "{name} draws nothing");
        }
    }
}
