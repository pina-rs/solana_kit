<!-- {@upstreamSummary} -->
- Upstream `@solana/kit` version in the pinned reference: `{{ upstream.parity[0].upstream_kit }}`
- Pinned Dart reference: [`{{ references.dart.repository | replace("https://github.com/", "") }}`]({{ references.dart.repository }}) at [`{{ upstream.parity[0].reference[0:12] }}`]({{ references.dart.repository }}/tree/{{ upstream.parity[0].reference }})
- This Rust SDK ports that reference; it does not compare itself to `@solana/kit` directly and makes no direct compatibility claim.
<!-- {/upstreamSummary} -->

<!-- {@upstreamParity} -->
| `solana_kit` workspace | `@solana/kit` | Pinned Dart commit | Released |
| --- | --- | --- | --- |
{% for row in upstream.parity %}| `{{ row.version }}` | `{{ row.upstream_kit }}` | `{{ row.reference[0:12] }}` | {% if row.released == "unreleased" %}_unreleased_{% else %}{{ row.released }}{% endif %} |
{% endfor %}
<!-- {/upstreamParity} -->

<!-- {@upstreamPins} -->
| Upstream repository | Pin | Used by |
| --- | --- | --- |
| [`{{ references.dart.upstream_kit_repository | replace("https://github.com/", "") }}`]({{ references.dart.upstream_kit_repository }}) | `v{{ references.dart.upstream_kit_reference }}` | all crates (upstream marker) |
{% for name in references.sources %}{% set source = references[name] %}| [`{{ source.repository | replace("https://github.com/", "") }}`]({{ source.repository }}) | {% if source.release %}`{{ source.release }}`{% if source.commit %} (`{{ source.commit[0:12] }}`){% endif %}{% else %}`{{ (source.selected_commit or source.commit)[0:12] }}`{% endif %} | {% if source.crates == ["*"] %}all crates{% else %}{% for crate in source.crates %}`{{ crate }}`{% if not loop.last %}, {% endif %}{% endfor %}{% endif %}{% if source.role %} ({{ source.role }}){% endif %} |
{% endfor %}
{% for name in references.sources %}{% set source = references[name] %}{% if source.selected_commit %}{% for crate in source.crates %}`{{ crate }}`{% if not loop.last %}, {% endif %}{% endfor %} uses the selected commit [`{{ source.selected_commit[0:12] }}`]({{ source.repository }}/tree/{{ source.selected_commit }}); the reference findings inspected [`{{ source.commit[0:12] }}`]({{ source.repository }}/tree/{{ source.commit }}).
{% endif %}{% endfor %}
<!-- {/upstreamPins} -->
