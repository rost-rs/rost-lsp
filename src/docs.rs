/// Comprehensive documentation registry for the .ROST language.
/// Provides rich markdown documentation, categorizations, and completion snippets.

pub struct DocEntry {
    pub symbol: &'static str,
    pub category: &'static str,
    pub summary: &'static str,
    pub detail_md: &'static str,
    pub snippet: &'static str,
}

pub static DIRECTIVES: &[DocEntry] = &[
    DocEntry {
        symbol: "@theme",
        category: "Directive",
        summary: "Defines the active visual theme identifier for the styling scope.",
        detail_md: "### `@theme` `[Directive]`\n\nSets the visual design theme token name.\n\n**Example:**\n```rost\n@theme \"cyber-dark\"\n```",
        snippet: "@theme \"${1:dark-nebula}\"",
    },
    DocEntry {
        symbol: "@version",
        category: "Directive",
        summary: "Specifies semantic version metadata for the stylesheet.",
        detail_md: "### `@version` `[Directive]`\n\nDeclares the semver specification of the stylesheet.\n\n**Example:**\n```rost\n@version \"1.0.0\"\n```",
        snippet: "@version \"${1:1.0.0}\"",
    },
    DocEntry {
        symbol: "@author",
        category: "Directive",
        summary: "Declares the stylesheet author or organization maintainer.",
        detail_md: "### `@author` `[Directive]`\n\nAuthor or design system organization declaration.\n\n**Example:**\n```rost\n@author \"utilrs\"\n```",
        snippet: "@author \"${1:utilrs}\"",
    },
    DocEntry {
        symbol: "@engine",
        category: "Directive",
        summary: "Target UI engine runtime (egui, gpui, iced, slint, bevy).",
        detail_md: "### `@engine` `[Directive]`\n\nSpecifies the intended backend rendering pipeline.\n\n**Example:**\n```rost\n@engine \"gpui\"\n```",
        snippet: "@engine \"${1:gpui}\"",
    },
    DocEntry {
        symbol: "@palette",
        category: "Directive",
        summary: "Root color palette dictionary block.",
        detail_md: "### `@palette` `[Directive]`\n\nDefines a global color token palette dictionary block.\n\n**Example:**\n```rost\n@palette {\n    bg: #1e1e2e,\n    accent: #cba6f7\n}\n```",
        snippet: "@palette {\n    bg_primary: ${1:#1e1e2e},\n    accent: ${2:#cba6f7},\n    text_main: ${3:#cdd6f4},\n    $0\n}",
    },
    DocEntry {
        symbol: "@rule",
        category: "Directive",
        summary: "Declares an AST linting or static analysis rule.",
        detail_md: "### `@rule` `[Directive]`\n\nDefines an architectural or lint verification rule.\n\n**Example:**\n```rost\n@rule \"enforce-tokens\"\n```",
        snippet: "@rule \"${1:rule-name}\"",
    },
    DocEntry {
        symbol: "@import",
        category: "Directive",
        summary: "Imports and merges an external .rost stylesheet.",
        detail_md: "### `@import` `[Directive]`\n\nImports external design tokens or stylesheets.\n\n**Example:**\n```rost\n@import \"tokens/palette.rost\"\n```",
        snippet: "@import \"${1:path/to/file.rost}\"",
    },
    DocEntry {
        symbol: "@extends",
        category: "Directive",
        summary: "Inherits declarations from a base component or theme.",
        detail_md: "### `@extends` `[Directive]`\n\nInherits rules and properties from a parent selector.\n\n**Example:**\n```rost\n@extends \"#button.base\"\n```",
        snippet: "@extends \"${1:#button.base}\"",
    },
    DocEntry {
        symbol: "@breakpoint",
        category: "Directive",
        summary: "Declares a responsive media breakpoint scope.",
        detail_md: "### `@breakpoint` `[Directive]`\n\nDefines layout variants for specific display dimensions.\n\n**Example:**\n```rost\n@breakpoint \"compact\" {\n    width: 100%,\n}\n```",
        snippet: "@breakpoint \"${1:compact}\" {\n    $0\n}",
    },
];

pub static PROPERTIES: &[DocEntry] = &[
    DocEntry {
        symbol: "width",
        category: "Layout Property",
        summary: "Horizontal dimension of the component bounding box.",
        detail_md: "### `width` `[Layout Property]`\n\nSpecifies the horizontal width dimension.\n\n**Units:** `px`, `rem`, `em`, `%`, `vw`\n**Example:** `width: 140px,`",
        snippet: "width: ${1:140px},",
    },
    DocEntry {
        symbol: "height",
        category: "Layout Property",
        summary: "Vertical dimension of the component bounding box.",
        detail_md: "### `height` `[Layout Property]`\n\nSpecifies the vertical height dimension.\n\n**Units:** `px`, `rem`, `em`, `%`, `vh`\n**Example:** `height: 44px,`",
        snippet: "height: ${1:44px},",
    },
    DocEntry {
        symbol: "min_width",
        category: "Constraint Property",
        summary: "Minimum allowable horizontal width boundary.",
        detail_md: "### `min_width` `[Constraint Property]`\n\nClamps width to never shrink below this limit.\n\n**Units:** `px`, `rem`, `em`, `%`, `vw`\n**Example:** `min_width: 80px,`",
        snippet: "min_width: ${1:80px},",
    },
    DocEntry {
        symbol: "max_width",
        category: "Constraint Property",
        summary: "Maximum allowable horizontal width boundary.",
        detail_md: "### `max_width` `[Constraint Property]`\n\nClamps width to never expand beyond this limit.\n\n**Units:** `px`, `rem`, `em`, `%`, `vw`\n**Example:** `max_width: 600px,`",
        snippet: "max_width: ${1:600px},",
    },
    DocEntry {
        symbol: "min_height",
        category: "Constraint Property",
        summary: "Minimum allowable vertical height boundary.",
        detail_md: "### `min_height` `[Constraint Property]`\n\nClamps height to never shrink below this limit.\n\n**Units:** `px`, `rem`, `em`, `%`, `vh`\n**Example:** `min_height: 32px,`",
        snippet: "min_height: ${1:32px},",
    },
    DocEntry {
        symbol: "max_height",
        category: "Constraint Property",
        summary: "Maximum allowable vertical height boundary.",
        detail_md: "### `max_height` `[Constraint Property]`\n\nClamps height to never expand beyond this limit.\n\n**Units:** `px`, `rem`, `em`, `%`, `vh`\n**Example:** `max_height: 800px,`",
        snippet: "max_height: ${1:800px},",
    },
    DocEntry {
        symbol: "bg",
        category: "Color Property",
        summary: "Background surface fill color.",
        detail_md: "### `bg` `[Color Property]`\n\nSpecifies background surface color.\n\n**Syntax:** `#RRGGBB`, `#RGB`, `#RRGGBBAA`\n**Example:** `bg: #1e1e2e,`",
        snippet: "bg: ${1:#1e1e2e},",
    },
    DocEntry {
        symbol: "color",
        category: "Color Property",
        summary: "Primary typography and foreground glyph color.",
        detail_md: "### `color` `[Color Property]`\n\nSpecifies foreground typography color.\n\n**Syntax:** `#RRGGBB`, `#RGB`, `#RRGGBBAA`\n**Example:** `color: #ffffff,`",
        snippet: "color: ${1:#ffffff},",
    },
    DocEntry {
        symbol: "border_radius",
        category: "Shape Property",
        summary: "Rounds the corners of the element bounding box.",
        detail_md: "### `border_radius` `[Shape Property]`\n\nDefines curvature radius for corner rounding.\n\n**Units:** `px`, `%`\n**Example:** `border_radius: 8px,`",
        snippet: "border_radius: ${1:8px},",
    },
    DocEntry {
        symbol: "border_width",
        category: "Border Property",
        summary: "Stroke thickness of the outer boundary outline.",
        detail_md: "### `border_width` `[Border Property]`\n\nSpecifies outline stroke thickness.\n\n**Units:** `px`\n**Example:** `border_width: 1.5px,`",
        snippet: "border_width: ${1:1.5px},",
    },
    DocEntry {
        symbol: "border_color",
        category: "Border Property",
        summary: "Color stroke used for drawing element edges.",
        detail_md: "### `border_color` `[Border Property]`\n\nSpecifies stroke color of the element outline.\n\n**Syntax:** `#RRGGBB`\n**Example:** `border_color: #313244,`",
        snippet: "border_color: ${1:#313244},",
    },
    DocEntry {
        symbol: "padding",
        category: "Box Model Property",
        summary: "Inner spacing between element border and contents.",
        detail_md: "### `padding` `[Box Model Property]`\n\nInner padding vector.\n\n**Syntax:** `[all]` or `[vert, horiz]` or `[top, right, bottom, left]`\n**Example:** `padding [ 8px, 16px ],`",
        snippet: "padding [ ${1:8px}, ${2:16px} ],",
    },
    DocEntry {
        symbol: "margin",
        category: "Box Model Property",
        summary: "Outer spacing surrounding the exterior of the element.",
        detail_md: "### `margin` `[Box Model Property]`\n\nOuter spacing vector.\n\n**Syntax:** `[all]` or `[vert, horiz]` or `[top, right, bottom, left]`\n**Example:** `margin [ 0px, 8px ],`",
        snippet: "margin [ ${1:0px}, ${2:8px} ],",
    },
    DocEntry {
        symbol: "opacity",
        category: "Visual Property",
        summary: "Transparency alpha coefficient ranging from 0.0 to 1.0.",
        detail_md: "### `opacity` `[Visual Property]`\n\nAlpha opacity multiplier: `0.0` is invisible, `1.0` is fully opaque.\n\n**Example:** `opacity: 0.95,`",
        snippet: "opacity: ${1:0.95},",
    },
    DocEntry {
        symbol: "visibility",
        category: "Visual Property",
        summary: "Visibility state (visible, hidden, collapsed).",
        detail_md: "### `visibility` `[Visual Property]`\n\nControls whether element renders on the GPU screen pipeline.\n\n**Values:** `visible`, `hidden`, `collapsed`\n**Example:** `visibility: visible,`",
        snippet: "visibility: ${1:visible},",
    },
    DocEntry {
        symbol: "box_shadow",
        category: "Elevation Property",
        summary: "Drop shadow parameters [x_off, y_off, blur, spread, color].",
        detail_md: "### `box_shadow` `[Elevation Property]`\n\nDrop shadow parameters for card elevation.\n\n**Syntax:** `[x, y, blur, color]`\n**Example:** `box_shadow: [ 0px, 4px, 12px, #00000040 ],`",
        snippet: "box_shadow: [ ${1:0px}, ${2:4px}, ${3:12px}, ${4:#00000040} ],",
    },
    DocEntry {
        symbol: "font_size",
        category: "Typography Property",
        summary: "Typeface scale in pixels, rem, or pt.",
        detail_md: "### `font_size` `[Typography Property]`\n\nSets font glyph size.\n\n**Units:** `px`, `rem`, `pt`\n**Example:** `font_size: 14px,`",
        snippet: "font_size: ${1:14px},",
    },
    DocEntry {
        symbol: "font_weight",
        category: "Typography Property",
        summary: "Visual weight and thickness of glyph strokes.",
        detail_md: "### `font_weight` `[Typography Property]`\n\nThickness of font strokes.\n\n**Values:** `100`..`900`, `normal`, `bold`, `light`, `semibold`\n**Example:** `font_weight: 600,`",
        snippet: "font_weight: ${1:600},",
    },
    DocEntry {
        symbol: "font_family",
        category: "Typography Property",
        summary: "Font typeface family name or registered asset token.",
        detail_md: "### `font_family` `[Typography Property]`\n\nTypeface family identifier.\n\n**Example:** `font_family: \"JetBrains Mono\",`",
        snippet: "font_family: \"${1:JetBrains Mono}\",",
    },
    DocEntry {
        symbol: "line_height",
        category: "Typography Property",
        summary: "Vertical distance between baselines of text lines.",
        detail_md: "### `line_height` `[Typography Property]`\n\nLine height multiplier or pixel spacing.\n\n**Example:** `line_height: 1.4,`",
        snippet: "line_height: ${1:1.4},",
    },
    DocEntry {
        symbol: "letter_spacing",
        category: "Typography Property",
        summary: "Kerning spacing between individual characters.",
        detail_md: "### `letter_spacing` `[Typography Property]`\n\nSpacing between glyphs.\n\n**Units:** `px`, `em`\n**Example:** `letter_spacing: 0.5px,`",
        snippet: "letter_spacing: ${1:0.5px},",
    },
    DocEntry {
        symbol: "text_align",
        category: "Typography Property",
        summary: "Horizontal alignment of text within container.",
        detail_md: "### `text_align` `[Typography Property]`\n\nText alignment inside container.\n\n**Values:** `left`, `center`, `right`, `justify`\n**Example:** `text_align: center,`",
        snippet: "text_align: ${1:center},",
    },
    DocEntry {
        symbol: "display",
        category: "Layout Algorithm Property",
        summary: "Layout flow mode (flex, grid, block, none).",
        detail_md: "### `display` `[Layout Property]`\n\nRendering algorithm for child element layout.\n\n**Values:** `flex`, `grid`, `block`, `none`\n**Example:** `display: flex,`",
        snippet: "display: ${1:flex},",
    },
    DocEntry {
        symbol: "flex_direction",
        category: "Flexbox Property",
        summary: "Primary direction axis of flex children.",
        detail_md: "### `flex_direction` `[Flexbox Property]`\n\nDirection of items in flex containers.\n\n**Values:** `row`, `column`, `row_reverse`, `column_reverse`\n**Example:** `flex_direction: row,`",
        snippet: "flex_direction: ${1:row},",
    },
    DocEntry {
        symbol: "align_items",
        category: "Flexbox Property",
        summary: "Cross-axis alignment of children.",
        detail_md: "### `align_items` `[Flexbox Property]`\n\nAligns children along cross-axis.\n\n**Values:** `start`, `center`, `end`, `stretch`, `baseline`\n**Example:** `align_items: center,`",
        snippet: "align_items: ${1:center},",
    },
    DocEntry {
        symbol: "justify_content",
        category: "Flexbox Property",
        summary: "Main-axis distribution of free space between items.",
        detail_md: "### `justify_content` `[Flexbox Property]`\n\nDistributes free space along main-axis.\n\n**Values:** `start`, `center`, `end`, `space_between`, `space_around`\n**Example:** `justify_content: space_between,`",
        snippet: "justify_content: ${1:space_between},",
    },
    DocEntry {
        symbol: "gap",
        category: "Spacing Property",
        summary: "Gutter spacing between adjacent items in flex/grid.",
        detail_md: "### `gap` `[Spacing Property]`\n\nUniform spacing between sibling children.\n\n**Units:** `px`, `rem`\n**Example:** `gap: 12px,`",
        snippet: "gap: ${1:12px},",
    },
    DocEntry {
        symbol: "overflow",
        category: "Clipping Property",
        summary: "Clipping behavior for contents overflowing boundary.",
        detail_md: "### `overflow` `[Clipping Property]`\n\nClipping behavior for overflowing contents.\n\n**Values:** `hidden`, `visible`, `scroll`, `auto`\n**Example:** `overflow: hidden,`",
        snippet: "overflow: ${1:hidden},",
    },
    DocEntry {
        symbol: "z_index",
        category: "Stacking Property",
        summary: "Integer 3D stacking order priority.",
        detail_md: "### `z_index` `[Stacking Property]`\n\nStacking order index for overlapping elements.\n\n**Example:** `z_index: 10,`",
        snippet: "z_index: ${1:10},",
    },
    DocEntry {
        symbol: "cursor",
        category: "Interaction Property",
        summary: "Pointer cursor icon displayed during hover.",
        detail_md: "### `cursor` `[Interaction Property]`\n\nCursor shape when pointer hovers over element.\n\n**Values:** `pointer`, `default`, `grab`, `text`, `not-allowed`\n**Example:** `cursor: pointer,`",
        snippet: "cursor: ${1:pointer},",
    },
    DocEntry {
        symbol: "transition",
        category: "Animation Property",
        summary: "Animated interpolation parameters (duration and easing).",
        detail_md: "### `transition` `[Animation Property]`\n\nSmooth state transition timing.\n\n**Example:** `transition: 150ms ease,`",
        snippet: "transition: ${1:150ms ease},",
    },
    DocEntry {
        symbol: "transform",
        category: "Transform Property",
        summary: "2D affine transformation (scale, rotate, translate).",
        detail_md: "### `transform` `[Transform Property]`\n\nGeometric transform applied to element.\n\n**Example:** `transform: scale(1.05),`",
        snippet: "transform: ${1:scale(1.05)},",
    },
    DocEntry {
        symbol: "state",
        category: "State Machine Enum",
        summary: "Declares active pseudo-state variants and interactive flags.",
        detail_md: "### `state` `[State Machine Enum]`\n\nDeclares interactive pseudo-class variants.\n\n**Syntax:** `state | idle, hover, active, disabled |`\n**Example:** `state | idle, hover, active |`",
        snippet: "state | ${1:idle}, ${2:hover}, ${3:active}, ${4:disabled} |, ",
    },
    DocEntry {
        symbol: "on_click",
        category: "Reactive Event Binding",
        summary: "Dispatches callback or message on mouse click / tap.",
        detail_md: "### `on_click` `[Reactive Event Binding]`\n\nDispatches UI event upon click.\n\n**Example:** `on_click = emit_submit(id: 42)`",
        snippet: "on_click = ${1:emit_event()}",
    },
    DocEntry {
        symbol: "on_hover",
        category: "Reactive Event Binding",
        summary: "Triggers animation or callback on pointer enter/leave.",
        detail_md: "### `on_hover` `[Reactive Event Binding]`\n\nDispatches UI event upon pointer hover.\n\n**Example:** `on_hover = animate_scale(1.05)`",
        snippet: "on_hover = ${1:animate_scale(1.05)}",
    },
    DocEntry {
        symbol: "on_change",
        category: "Reactive Event Binding",
        summary: "Dispatched when input text or slider value is modified.",
        detail_md: "### `on_change` `[Reactive Event Binding]`\n\nDispatches UI event upon value change.\n\n**Example:** `on_change = set_value($val)`",
        snippet: "on_change = ${1:update_value($val)}",
    },
    DocEntry {
        symbol: "on_submit",
        category: "Reactive Event Binding",
        summary: "Dispatched upon form submission or enter key press.",
        detail_md: "### `on_submit` `[Reactive Event Binding]`\n\nDispatches form submit action.\n\n**Example:** `on_submit = submit_form()`",
        snippet: "on_submit = ${1:submit_form()}",
    },
    DocEntry {
        symbol: "title",
        category: "Window / Header Property",
        summary: "Human-readable window header or component title string.",
        detail_md: "### `title` `[Window / Header Property]`\n\nSpecifies the title string for the OS window frame or header.\n\n**Example:** `title: \"My Native App\",`",
        snippet: "title: \"${1:Application Title}\",",
    },
    DocEntry {
        symbol: "subtitle",
        category: "Typography Property",
        summary: "Secondary descriptive subtitle or caption text.",
        detail_md: "### `subtitle` `[Typography Property]`\n\nSecondary typography subtitle or descriptive caption string.\n\n**Example:** `subtitle: \"Declarative styling for Rust\",`",
        snippet: "subtitle: \"${1:Descriptive Subtitle}\",",
    },
    DocEntry {
        symbol: "label",
        category: "Content Property",
        summary: "Textual label or badge caption displayed inside component.",
        detail_md: "### `label` `[Content Property]`\n\nTextual label displayed inside buttons, badges, or chips.\n\n**Example:** `label: \"Submit\",`",
        snippet: "label: \"${1:Button Label}\",",
    },
    DocEntry {
        symbol: "text_color",
        category: "Color Property",
        summary: "Alias for color: foreground typography color.",
        detail_md: "### `text_color` `[Color Property]`\n\nSpecifies text color inside the element.\n\n**Syntax:** `#RRGGBB`\n**Example:** `text_color: #ffffff,`",
        snippet: "text_color: ${1:#ffffff},",
    },
    DocEntry {
        symbol: "elevation",
        category: "Elevation Property",
        summary: "Z-elevation depth in virtual pixels determining shadow depth.",
        detail_md: "### `elevation` `[Elevation Property]`\n\nVirtual elevation level (dp) determining drop shadow depth and layer priority.\n\n**Example:** `elevation: 8,`",
        snippet: "elevation: ${1:4},",
    },
    DocEntry {
        symbol: "margin_top",
        category: "Box Model Property",
        summary: "Top outer spacing margin.",
        detail_md: "### `margin_top` `[Box Model Property]`\n\nOuter spacing on the top edge.\n\n**Units:** `px`, `rem`\n**Example:** `margin_top: 16px,`",
        snippet: "margin_top: ${1:16px},",
    },
    DocEntry {
        symbol: "margin_bottom",
        category: "Box Model Property",
        summary: "Bottom outer spacing margin.",
        detail_md: "### `margin_bottom` `[Box Model Property]`\n\nOuter spacing on the bottom edge.\n\n**Units:** `px`, `rem`\n**Example:** `margin_bottom: 16px,`",
        snippet: "margin_bottom: ${1:16px},",
    },
    DocEntry {
        symbol: "margin_left",
        category: "Box Model Property",
        summary: "Left outer spacing margin.",
        detail_md: "### `margin_left` `[Box Model Property]`\n\nOuter spacing on the left edge.\n\n**Units:** `px`, `rem`\n**Example:** `margin_left: 12px,`",
        snippet: "margin_left: ${1:12px},",
    },
    DocEntry {
        symbol: "margin_right",
        category: "Box Model Property",
        summary: "Right outer spacing margin.",
        detail_md: "### `margin_right` `[Box Model Property]`\n\nOuter spacing on the right edge.\n\n**Units:** `px`, `rem`\n**Example:** `margin_right: 12px,`",
        snippet: "margin_right: ${1:12px},",
    },
    DocEntry {
        symbol: "padding_top",
        category: "Box Model Property",
        summary: "Top inner spacing padding.",
        detail_md: "### `padding_top` `[Box Model Property]`\n\nInner spacing on the top edge.\n\n**Units:** `px`, `rem`\n**Example:** `padding_top: 12px,`",
        snippet: "padding_top: ${1:12px},",
    },
    DocEntry {
        symbol: "padding_bottom",
        category: "Box Model Property",
        summary: "Bottom inner spacing padding.",
        detail_md: "### `padding_bottom` `[Box Model Property]`\n\nInner spacing on the bottom edge.\n\n**Units:** `px`, `rem`\n**Example:** `padding_bottom: 12px,`",
        snippet: "padding_bottom: ${1:12px},",
    },
    DocEntry {
        symbol: "padding_left",
        category: "Box Model Property",
        summary: "Left inner spacing padding.",
        detail_md: "### `padding_left` `[Box Model Property]`\n\nInner spacing on the left edge.\n\n**Units:** `px`, `rem`\n**Example:** `padding_left: 16px,`",
        snippet: "padding_left: ${1:16px},",
    },
    DocEntry {
        symbol: "padding_right",
        category: "Box Model Property",
        summary: "Right inner spacing padding.",
        detail_md: "### `padding_right` `[Box Model Property]`\n\nInner spacing on the right edge.\n\n**Units:** `px`, `rem`\n**Example:** `padding_right: 16px,`",
        snippet: "padding_right: ${1:16px},",
    },
];

pub static STATES: &[DocEntry] = &[
    DocEntry {
        symbol: "idle",
        category: "Pseudo State",
        summary: "Default resting state with no user interaction.",
        detail_md: "### `idle` `[Pseudo State]`\n\nDefault resting state of the UI element.",
        snippet: "idle",
    },
    DocEntry {
        symbol: "hover",
        category: "Pseudo State",
        summary: "Pointer cursor is within element bounding box.",
        detail_md: "### `hover` `[Pseudo State]`\n\nActive when mouse pointer hovers over element boundary.",
        snippet: "hover",
    },
    DocEntry {
        symbol: "active",
        category: "Pseudo State",
        summary: "Element is actively held down or pressed.",
        detail_md: "### `active` `[Pseudo State]`\n\nActive during mouse down-press or touch contact.",
        snippet: "active",
    },
    DocEntry {
        symbol: "disabled",
        category: "Pseudo State",
        summary: "Element is non-interactive and dimmed.",
        detail_md: "### `disabled` `[Pseudo State]`\n\nDisables pointer events and visually dims the component.",
        snippet: "disabled",
    },
    DocEntry {
        symbol: "focused",
        category: "Pseudo State",
        summary: "Element holds keyboard or accessibility focus.",
        detail_md: "### `focused` `[Pseudo State]`\n\nActive when element holds focus for keyboard inputs.",
        snippet: "focused",
    },
    DocEntry {
        symbol: "selected",
        category: "Pseudo State",
        summary: "Item is chosen in a list or toggle group.",
        detail_md: "### `selected` `[Pseudo State]`\n\nActive when item is selected in a radio group or collection.",
        snippet: "selected",
    },
    DocEntry {
        symbol: "pressed",
        category: "Pseudo State",
        summary: "Mechanical down-press phase before mouse release.",
        detail_md: "### `pressed` `[Pseudo State]`\n\nRepresents the physical depression phase of a push button.",
        snippet: "pressed",
    },
];

pub static UNITS: &[DocEntry] = &[
    DocEntry {
        symbol: "px",
        category: "Unit",
        summary: "Pixels: absolute device-independent screen coordinate units.",
        detail_md: "### `px` `[Unit]`\n\nDevice-independent pixels mapped directly to GPU framebuffer coordinates.",
        snippet: "px",
    },
    DocEntry {
        symbol: "rem",
        category: "Unit",
        summary: "Root EM: relative to theme base font size scale.",
        detail_md: "### `rem` `[Unit]`\n\nRelative units scaled proportional to the theme's root font size (usually 16px).",
        snippet: "rem",
    },
    DocEntry {
        symbol: "em",
        category: "Unit",
        summary: "Relative to current element font size.",
        detail_md: "### `em` `[Unit]`\n\nRelative units scaled against the parent element's active font size.",
        snippet: "em",
    },
    DocEntry {
        symbol: "%",
        category: "Unit",
        summary: "Percentage relative to parent container dimension.",
        detail_md: "### `%` `[Unit]`\n\nPercentage fraction computed relative to parent layout boundary.",
        snippet: "%",
    },
    DocEntry {
        symbol: "vh",
        category: "Unit",
        summary: "Viewport Height: 1vh equals 1% of total window height.",
        detail_md: "### `vh` `[Unit]`\n\nViewport height units relative to total application window height.",
        snippet: "vh",
    },
    DocEntry {
        symbol: "vw",
        category: "Unit",
        summary: "Viewport Width: 1vw equals 1% of total window width.",
        detail_md: "### `vw` `[Unit]`\n\nViewport width units relative to total application window width.",
        snippet: "vw",
    },
    DocEntry {
        symbol: "ms",
        category: "Unit",
        summary: "Milliseconds for animation and transition timing.",
        detail_md: "### `ms` `[Unit]`\n\nMillisecond time interval for animation and easing durations.",
        snippet: "ms",
    },
    DocEntry {
        symbol: "s",
        category: "Unit",
        summary: "Seconds for animation duration.",
        detail_md: "### `s` `[Unit]`\n\nSeconds time duration.",
        snippet: "s",
    },
    DocEntry {
        symbol: "deg",
        category: "Unit",
        summary: "Degrees for geometric rotation.",
        detail_md: "### `deg` `[Unit]`\n\nDegrees angle for transform rotations and gradient angles.",
        snippet: "deg",
    },
];

pub static SELECTORS: &[DocEntry] = &[
    DocEntry {
        symbol: "#window",
        category: "Selector Block",
        summary: "Root application window container and styling bounds.",
        detail_md: "### `#window` `[Selector Block]`\n\nDefines root OS window parameters (dimensions, background, title, decorations).\n\n```rost\n#window {\n    width: 680px,\n    height: 520px,\n    bg: #11111b,\n}\n```",
        snippet: "#window {\n    width: ${1:680px},\n    height: ${2:520px},\n    bg: ${3:#11111b},\n    $0\n}",
    },
    DocEntry {
        symbol: "#button",
        category: "Selector Block",
        summary: "Interactive push button component styling block.",
        detail_md: "### `#button` `[Selector Block]`\n\nInteractive push button component styling scope.\n\n```rost\n#button.primary {\n    width: 140px,\n    height: 44px,\n    bg: #1e1e2e,\n    border_radius: 8px,\n}\n```",
        snippet: "#${1:button}.${2:primary} {\n    width: ${3:140px},\n    height: ${4:44px},\n    bg: ${5:#1e1e2e},\n    border_radius: ${6:8px},\n    state | idle, hover, active |,\n    $0\n}",
    },
    DocEntry {
        symbol: "#label",
        category: "Selector Block",
        summary: "Read-only text label and typography display block.",
        detail_md: "### `#label` `[Selector Block]`\n\nRead-only typography display styling block.",
        snippet: "#label {\n    color: ${1:#cdd6f4},\n    font_size: ${2:14px},\n    font_weight: ${3:500},\n    $0\n}",
    },
    DocEntry {
        symbol: "#card",
        category: "Selector Block",
        summary: "Raised surface container with elevation and padding.",
        detail_md: "### `#card` `[Selector Block]`\n\nRaised or bordered surface container.",
        snippet: "#card {\n    bg: ${1:#181825},\n    border_radius: ${2:12px},\n    padding [ ${3:16px}, ${4:20px} ],\n    $0\n}",
    },
    DocEntry {
        symbol: "#slider",
        category: "Selector Block",
        summary: "Numeric continuous range slider control.",
        detail_md: "### `#slider` `[Selector Block]`\n\nContinuous numeric value slider styling block.",
        snippet: "#slider {\n    width: ${1:200px},\n    height: ${2:8px},\n    bg: ${3:#313244},\n    $0\n}",
    },
    DocEntry {
        symbol: "#input",
        category: "Selector Block",
        summary: "Text entry and form input editor field.",
        detail_md: "### `#input` `[Selector Block]`\n\nText input field styling block.",
        snippet: "#input {\n    width: ${1:240px},\n    height: ${2:40px},\n    bg: ${3:#181825},\n    border_radius: ${4:6px},\n    color: ${5:#cdd6f4},\n    $0\n}",
    },
    DocEntry {
        symbol: "#checkbox",
        category: "Selector Block",
        summary: "Boolean toggle checkbox control.",
        detail_md: "### `#checkbox` `[Selector Block]`\n\nBoolean toggle checkbox indicator.",
        snippet: "#checkbox {\n    width: ${1:20px},\n    height: ${2:20px},\n    border_radius: ${3:4px},\n    state | checked, idle |,\n    $0\n}",
    },
];

/// Looks up rich markdown documentation for any token, keyword, property, or directive.
pub fn lookup_docs(raw_word: &str) -> Option<String> {
    let word = raw_word.trim().trim_end_matches([':', ',', ';', '{', '}']);

    // Check hex colors (handled dynamically by caller)
    if word.starts_with('#') && word.len() > 1 && word[1..].chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }

    // Directives
    for item in DIRECTIVES {
        if item.symbol == word || item.symbol.strip_prefix('@') == Some(word) {
            return Some(item.detail_md.to_string());
        }
    }

    // Properties
    for item in PROPERTIES {
        if item.symbol == word {
            return Some(item.detail_md.to_string());
        }
    }

    // States
    for item in STATES {
        if item.symbol == word {
            return Some(item.detail_md.to_string());
        }
    }

    // Selectors
    for item in SELECTORS {
        if item.symbol == word || item.symbol.strip_prefix('#') == Some(word) {
            return Some(item.detail_md.to_string());
        }
    }

    // Units (exact or suffix after numeric value)
    for item in UNITS {
        if item.symbol == word {
            return Some(item.detail_md.to_string());
        }
    }

    // Check if word is a number + unit (e.g. 140px, 1.5rem)
    let unit_candidate = word.trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c == '-');
    if !unit_candidate.is_empty() {
        for item in UNITS {
            if item.symbol == unit_candidate {
                return Some(item.detail_md.to_string());
            }
        }
    }

    // Component selector with variant or custom selector (e.g. #button.primary, #header)
    if word.starts_with('#') {
        let clean = &word[1..];
        return Some(format!(
            "### `#{}` `[Component Selector]`\n\nScoped style block for UI component or container `{}`.\n\n```rost\n{} {{\n    /* properties */\n}}\n```",
            clean, clean, word
        ));
    }

    // Dynamic palette background tokens (e.g. bg_window, bg_card, bg_surface)
    if word.starts_with("bg_") {
        return Some(format!(
            "### `{}` `[Palette Token / Background]`\n\nSurface fill color token for `{}`. Defined in `#palette` and consumed by UI renderers.",
            word, &word[3..]
        ));
    }

    // Dynamic palette text tokens (e.g. text_primary, text_secondary, text_dark)
    if word.starts_with("text_") {
        return Some(format!(
            "### `{}` `[Palette Token / Typography]`\n\nTypography text color token for `{}`.",
            word, &word[5..]
        ));
    }

    // Dynamic accent tokens (e.g. accent_warning, accent_danger)
    if word.starts_with("accent") {
        return Some(format!(
            "### `{}` `[Palette Token / Accent]`\n\nAccentuation or semantic status highlight color token.",
            word
        ));
    }

    // Generic fallback for any identifier property in .rost
    if !word.is_empty() && word.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Some(format!(
            "### `{}` `[ROST Attribute / Field]`\n\nDeclarative property or configuration attribute parsed with zero-allocation.",
            word
        ));
    }

    None
}
