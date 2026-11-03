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

/// A stable seed derived from `text` (FNV-1a), for ink that should differ
/// between elements but never between builds.
pub fn seed_for(text: &str) -> i64 {
    let mut hash: u32 = 0x811c_9dc5;

    for unit in text.encode_utf16() {
        hash = (hash ^ u32::from(unit)).wrapping_mul(0x0100_0193) & 0x7fff_ffff;
    }

    i64::from(hash % 100_000 + 1)
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
        Shape::Underline => generator.curve(&[
            Point::new(inset, height * 0.72),
            Point::new(width * 0.35, height * 0.62),
            Point::new(width * 0.7, height * 0.7),
            Point::new(width - inset, height * 0.58),
        ]),
        Shape::Loop => {
            let points = (0..=14)
                .map(|step| {
                    let angle = -PI * 0.1 + f64::from(step) * (PI * 2.15 / 14.0);
                    let wander = 1.0 + 0.05 * f64::from(step % 3);
                    Point::new(
                        width / 2.0 + (width / 2.0 - inset) / 1.1 * angle.cos() * wander,
                        height / 2.0 + (height / 2.0 - inset) * 0.8 * angle.sin(),
                    )
                })
                .collect::<Vec<_>>();
            generator.curve(&points)
        }
    };

    RoughSvgPaths::from_drawable(&drawable)
}

/// A standalone SVG file for a CSS mask: every group drawn in black.
pub fn mask(paths: &RoughSvgPaths, canvas: Canvas) -> String {
    let elements = paths.to_path_elements(&PathElementOptions {
        stroke: "#000",
        fill: "#000",
        stroke_width: 2.0,
        sketch_width: 1.5,
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
        stroke_width: 2.0,
        sketch_width: 1.5,
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

    for (data, class, width) in [
        (&paths.sketch, "sk-ink-hatch", 1.5),
        (&paths.outline, "sk-ink-outline", 2.0),
    ] {
        if !data.is_empty() {
            let paint = if class == "sk-ink-hatch" {
                "var(--sk-fill,currentColor)"
            } else {
                "currentColor"
            };
            let _ = write!(
                svg,
                r#"<path class="{class}" d="{data}" style="fill:none;stroke:{paint};stroke-width:{}px;stroke-linecap:round;stroke-linejoin:round" vector-effect="non-scaling-stroke"/>"#,
                number(width)
            );
        }
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
            width: 2.0,
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
///
/// `label` gives the drawing an accessible name; without one it is hidden
/// from assistive technology.
pub fn doodle(
    width: f64,
    height: f64,
    marks: &[Mark],
    palette: Palette,
    class: Option<&str>,
    label: Option<&str>,
) -> String {
    let mut svg = String::from("<svg");

    if palette != Palette::Page {
        svg.push_str(r#" xmlns="http://www.w3.org/2000/svg""#);
    }
    if let Some(class) = class {
        let _ = write!(svg, r#" class="{class}""#);
    }
    let _ = write!(
        svg,
        r#" viewBox="0 0 {w} {h}" width="{w}" height="{h}" focusable="false""#,
        w = number(width),
        h = number(height),
    );
    match label {
        Some(label) => {
            let _ = write!(svg, r#" role="img" aria-label="{label}""#);
        }
        None => svg.push_str(r#" aria-hidden="true""#),
    }
    svg.push('>');

    for mark in marks {
        let paths = RoughSvgPaths::from_drawable(&mark.drawable);
        let fill = palette.paint(mark.fill.unwrap_or(mark.stroke));
        let stroke = palette.paint(mark.stroke);

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

    svg.push_str("</svg>");
    svg
}

/// The solana_kit mark, drawn by hand on a 42 by 45 canvas: Solana's three
/// bars in mint, sky, and lilac, held in a pair of square brackets.
pub fn brand_mark(seed: i64) -> Vec<Mark> {
    const BARS: [([(f64, f64); 4], Tone); 3] = [
        (
            [(13.0, 9.0), (35.0, 9.0), (29.0, 16.0), (7.0, 16.0)],
            Tone::Mint,
        ),
        (
            [(7.0, 19.0), (29.0, 19.0), (35.0, 26.0), (13.0, 26.0)],
            Tone::Sky,
        ),
        (
            [(13.0, 29.0), (35.0, 29.0), (29.0, 36.0), (7.0, 36.0)],
            Tone::Lilac,
        ),
    ];
    let mirror = |x: f64, left: bool| if left { x } else { 42.0 - x };
    let mut marks = Vec::new();

    for (index, (points, tone)) in (0_i64..).zip(BARS) {
        let points = points.map(|(x, y)| Point::new(x, y));
        let drawable = pen(seed + index, 0.6, FillStyle::Solid, 5.0).polygon(&points);
        marks.push(Mark::new(drawable, Tone::Ink).fill(tone).width(1.6));
    }

    for (index, left) in (0_i64..).zip([true, false]) {
        let points = [
            Point::new(mirror(5.0, left), 2.0),
            Point::new(mirror(1.5, left), 2.5),
            Point::new(mirror(1.5, left), 42.5),
            Point::new(mirror(5.0, left), 43.0),
        ];
        let drawable = pen(seed + 5 + index, 0.5, FillStyle::None, 5.0).linear_path(&points, false);
        marks.push(Mark::new(drawable, Tone::Ink).width(2.4));
    }

    marks
}

/// How many seeded variants of each mask shape the stylesheet cycles through.
const VARIANTS: i64 = 4;

/// Every file in `theme/ink`, by name: the masks the stylesheet uses and the
/// brand mark on light and dark paper.
pub fn files() -> Vec<(String, String)> {
    let mut files = Vec::new();
    let mut add = |name: String, svg: String| files.push((name, svg));

    for variant in 1..=VARIANTS {
        let seed = variant * 11;
        let shapes = [
            ("box", Shape::Rectangle, Canvas::BOX, 1.8),
            ("card", Shape::Rectangle, Canvas::CARD, 1.8),
            ("circle", Shape::Ellipse, Canvas::SQUARE, 1.0),
            ("rule", Shape::Rule, Canvas::RULE, 1.4),
            ("edge", Shape::Edge, Canvas::EDGE, 1.4),
            ("underline", Shape::Underline, Canvas::WORD, 1.2),
            ("loop", Shape::Loop, Canvas::WORD, 1.0),
        ];

        for (name, shape, canvas, roughness) in shapes {
            let paths = draw(shape, canvas, seed, roughness, FillStyle::None);
            add(format!("{name}-{variant}.svg"), mask(&paths, canvas));
        }

        let hatch = draw(Shape::Rectangle, Canvas::BOX, seed, 1.0, FillStyle::Hachure);
        add(
            format!("hatch-{variant}.svg"),
            hatch_mask(&hatch, Canvas::BOX),
        );

        let outline = draw(
            Shape::Rectangle,
            Canvas::CARD,
            seed + 3,
            1.5,
            FillStyle::None,
        );
        add(
            format!("frame-{variant}-light.svg"),
            frame(&outline, Canvas::CARD, Tone::Ink.light()),
        );
        add(
            format!("frame-{variant}-dark.svg"),
            frame(&outline, Canvas::CARD, Tone::Ink.dark()),
        );
    }

    let mark = brand_mark(11);
    add(
        "mark-light.svg".to_owned(),
        doodle(42.0, 45.0, &mark, Palette::Light, None, Some("solana_kit")),
    );
    add(
        "mark-dark.svg".to_owned(),
        doodle(42.0, 45.0, &mark, Palette::Dark, None, Some("solana_kit")),
    );

    files
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
