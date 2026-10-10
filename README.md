# rost-lsp

A high-performance **Language Server Protocol (LSP)** server for the **`.rost`** declarative stylesheet and configuration language, built with Rust and powered by [`utilrs`](../utilrs).

---

## Features

- **⚡ Zero-Latency Real-Time Diagnostics:** Validates brace matching, brackets, pipes, unclosed strings, and property syntax on every keystroke.
- **💡 Smart Autocompletion:**
  - **Directives:** `@theme`, `@version`, `@author`, `@engine`, `@palette`, `@rule`, `@import`, `@extends`, `@breakpoint`.
  - **Properties:** Dimensions, constraints, box model, colors, typography, flexbox/layout, elevation, animations, and reactive bindings.
  - **Units:** `px`, `rem`, `em`, `%`, `vh`, `vw`, `ms`, `s`, `deg`, `pt`.
  - **Pseudo-state enums:** `state | idle, hover, active, disabled, focused, selected, pressed |`.
  - **Components:** `#window`, `#button.primary`, `#card`, `#palette`, `#input`, `#slider`, `#checkbox`, `#badge`.
- **📖 Comprehensive Hover Documentation:** Rich Markdown documentation for every single property, directive, unit, selector, and dynamic RGB hex color preview.
- **📐 Document Formatting:** Automated 4-space indentation and clean block styling.

---

## Installation

### 1. Automated Installation (Recommended)

Run the included installation script from within the repository:

```bash
git clone https://github.com/rost-rs/rost-lsp.git
cd rost-lsp
./install.sh
```

This compiles and installs `rost-lsp` to `~/.cargo/bin/rost-lsp` and ensures Neovim configuration is in place.

---

### 2. Manual Build and Install Binary

From the repository directory:

```bash
cd rost-lsp
cargo install --path .
```

This installs the binary `rost-lsp` to `~/.cargo/bin/rost-lsp`. Ensure `~/.cargo/bin` is in your `$PATH`.

Verify the installation:
```bash
which rost-lsp
```

---

## Language Reference & Tooling Dictionary

Every directive, property, pseudo-state, and unit is natively recognized and documented in `rost-lsp`:

### 1. Directives

| Directive | Description | Syntax Example |
| :--- | :--- | :--- |
| `@theme` | Sets active visual theme or token scope identifier | `@theme "cyber-dark"` |
| `@version` | Specifies semantic version metadata | `@version "1.0.0"` |
| `@author` | Declares author or organization maintainer | `@author "utilrs"` |
| `@engine` | Target native UI engine (`gpui`, `egui`, `iced`, `slint`, `bevy`) | `@engine "gpui"` |
| `@palette` | Root color token palette dictionary block | `@palette { bg: #1e1e2e }` |
| `@rule` | Declares an AST linting or static analysis rule | `@rule "enforce-tokens"` |
| `@import` | Imports and merges an external `.rost` file | `@import "tokens/palette.rost"` |
| `@extends` | Inherits rules from a parent component or theme block | `@extends "#button.base"` |
| `@breakpoint` | Declares responsive media query conditions | `@breakpoint "compact" { ... }` |

### 2. Properties

| Property | Category | Description | Supported Units / Values |
| :--- | :--- | :--- | :--- |
| `width`, `height` | Dimensions | Horizontal and vertical bounding box size | `px`, `rem`, `em`, `%`, `vw`, `vh` |
| `min_width`, `max_width` | Constraints | Clamp horizontal width boundaries | `px`, `rem`, `%`, `vw` |
| `min_height`, `max_height` | Constraints | Clamp vertical height boundaries | `px`, `rem`, `%`, `vh` |
| `bg` | Color | Background surface fill color | Hex `#RGB`, `#RRGGBB`, `#RRGGBBAA` |
| `color`, `text_color` | Color | Typography and foreground glyph color | Hex `#RGB`, `#RRGGBB` |
| `opacity` | Visual | Alpha transparency multiplier (0.0 - 1.0) | `0.0` .. `1.0` |
| `visibility` | Visual | Screen pipeline rendering visibility | `visible`, `hidden`, `collapsed` |
| `border_radius` | Shape | Curvature radius for rounded corners | `px`, `%` |
| `border_width` | Border | Outline stroke thickness | `px` |
| `border_color` | Border | Outline stroke color | Hex `#RRGGBB` |
| `padding` | Box Model | Inner spacing vector | `[all]`, `[v, h]`, `[t, r, b, l]` |
| `margin` | Box Model | Outer spacing vector | `[all]`, `[v, h]`, `[t, r, b, l]` |
| `margin_top`, `margin_bottom` | Box Model | Directional edge margins | `px`, `rem` |
| `margin_left`, `margin_right` | Box Model | Directional edge margins | `px`, `rem` |
| `padding_top`, `padding_bottom` | Box Model | Directional edge paddings | `px`, `rem` |
| `padding_left`, `padding_right` | Box Model | Directional edge paddings | `px`, `rem` |
| `elevation` | Elevation | Virtual depth level for shadow rendering | `4`, `8`, `12` |
| `box_shadow` | Elevation | Drop shadow parameters vector | `[ x, y, blur, color ]` |
| `font_size` | Typography | Typeface glyph scale | `px`, `rem`, `pt` |
| `font_weight` | Typography | Stroke thickness weight | `100`..`900`, `normal`, `bold` |
| `font_family` | Typography | Typeface font family asset token | `"JetBrains Mono"`, `"Inter"` |
| `line_height` | Typography | Vertical distance between text lines | `1.2`, `1.4`, `1.6` |
| `letter_spacing` | Typography | Kerning spacing between glyphs | `px`, `em` |
| `text_align` | Typography | Horizontal text alignment | `left`, `center`, `right`, `justify` |
| `display` | Layout | Container rendering flow algorithm | `flex`, `grid`, `block`, `none` |
| `flex_direction` | Flexbox | Main axis distribution direction | `row`, `column`, `row_reverse` |
| `align_items` | Flexbox | Cross-axis alignment of children | `start`, `center`, `end`, `stretch` |
| `justify_content` | Flexbox | Main-axis distribution of free space | `start`, `center`, `space_between` |
| `gap` | Spacing | Gutter spacing between sibling items | `px`, `rem` |
| `overflow` | Clipping | Content boundary overflow clipping | `hidden`, `visible`, `scroll` |
| `z_index` | Stacking | Integer layer stacking order priority | Integer index |
| `cursor` | Interaction | Mouse cursor pointer icon | `pointer`, `default`, `grab` |
| `transition` | Animation | State transition duration and easing | `150ms ease`, `200ms ease-in-out` |
| `transform` | Transform | 2D affine transformation matrix | `scale(1.05)`, `rotate(45deg)` |
| `title`, `subtitle` | Metadata | Header captions for windows and cards | String literals |
| `label` | Content | Text displayed inside buttons or badges | String literals |
| `state` | State Machine | Interactive pseudo-class variants enum | `\| idle, hover, active \|` |
| `on_click` | Reactive Binding | Event callback triggered on click / tap | `emit_event(id: 42)` |
| `on_hover` | Reactive Binding | Event callback triggered on hover enter | `animate_scale(1.05)` |
| `on_change` | Reactive Binding | Event callback triggered on input change | `update_value($val)` |
| `on_submit` | Reactive Binding | Event callback triggered on form submit | `submit_dialog()` |

### 3. Pseudo-States

- `idle`: Default resting state when no user interaction occurs.
- `hover`: Pointer cursor is within the element bounding box.
- `active`: Mouse button or touch contact is held down on the element.
- `disabled`: Element is non-interactive and visually dimmed.
- `focused`: Element holds keyboard or accessibility focus.
- `selected`: Item is active in a selection group or radio list.
- `pressed`: Mechanical down-press phase before mouse button release.

### 4. Units

- `px`: Device-independent pixels mapped directly to GPU coordinates.
- `rem`: Root EM relative to theme base font size scale.
- `em`: Relative to parent container font scale.
- `%`: Percentage relative to parent layout boundary.
- `vh`: Viewport height percent (1vh = 1% of total window height).
- `vw`: Viewport width percent (1vw = 1% of total window width).
- `ms`: Millisecond duration interval for transitions and animations.
- `s`: Seconds timing duration.
- `deg`: Angle degrees for rotations and gradients.

---

## Editor Configuration & File Icon Setup

### Neovim / LazyVim Complete Setup

In `~/.config/nvim/lua/plugins/rost.lua`:

```lua
-- 1. Native .rost filetype detection
vim.filetype.add({
  extension = {
    rost = "rost",
  },
})

-- 2. Auto-start Tree-sitter and rost-lsp
vim.api.nvim_create_autocmd("FileType", {
  pattern = "rost",
  callback = function(args)
    pcall(vim.treesitter.start, args.buf, "rost")
    vim.lsp.start({
      name = "rost-lsp",
      cmd = { "rost-lsp" },
      root_dir = vim.fs.root(args.buf, { ".git", "Cargo.toml" }) or vim.fn.getcwd(),
    })
  end,
})

-- 3. File Icon Setup (Vector SVG: assets/rost.svg)
-- Programmatic Nerd Font U+E68B to prevent broken/unprintable glyphs in text
local rost_icon = vim.fn.nr2char(0xe68b)

pcall(function()
  local mini = require("mini.icons")
  mini.setup({
    file = { [".rost"] = { glyph = rost_icon, hl = "MiniIconsOrange" } },
    filetype = { rost = { glyph = rost_icon, hl = "MiniIconsOrange" } },
    extension = { rost = { glyph = rost_icon, hl = "MiniIconsOrange" } },
  })
end)

pcall(function()
  local devicons = require("nvim-web-devicons")
  devicons.set_icon({
    rost = { icon = rost_icon, color = "#e55f28", cterm_color = "208", name = "Rost" },
  })
end)

return {
  -- mini.icons integration (LazyVim default for Neo-tree, Snacks, Bufferline)
  {
    "nvim-mini/mini.icons",
    opts = {
      file = { [".rost"] = { glyph = rost_icon, hl = "MiniIconsOrange" } },
      filetype = { rost = { glyph = rost_icon, hl = "MiniIconsOrange" } },
      extension = { rost = { glyph = rost_icon, hl = "MiniIconsOrange" } },
    },
  },

  -- Fallback for nvim-web-devicons
  {
    "nvim-tree/nvim-web-devicons",
    opts = {
      override = {
        rost = { icon = rost_icon, color = "#e55f28", cterm_color = "208", name = "Rost" },
      },
    },
  },

  -- Tree-sitter registration
  {
    "nvim-treesitter/nvim-treesitter",
    opts = function(_, opts)
      local parsers = require("nvim-treesitter.parsers")
      local parser_config = parsers.get_parser_configs and parsers.get_parser_configs() or parsers

      parser_config.rost = {
        install_info = {
          url = "https://github.com/rost-rs/tree-sitter-rost", -- or local clone path
          files = { "src/parser.c" },
          generate_requires_npm = false,
        },
        filetype = "rost",
      }
    end,
  },
}
```

---

### Helix Editor

In `~/.config/helix/languages.toml`:

```toml
[language-server.rost-lsp]
command = "rost-lsp"

[[language]]
name = "rost"
scope = "source.rost"
injection-regex = "rost"
file-types = ["rost"]
comment-token = "//"
language-servers = ["rost-lsp"]
indent = { tab-width = 4, unit = "    " }
```

---

### Zed Editor

In `~/.config/zed/settings.json`:

```json
{
  "languages": {
    "Rost": {
      "language_servers": ["rost-lsp"]
    }
  },
  "lsp": {
    "rost-lsp": {
      "binary": {
        "path": "rost-lsp"
      }
    }
  }
}
```

---

### Formatting via LSP

In Neovim / LazyVim:
```vim
:lua vim.lsp.buf.format()
```
Or simply press `<leader>cf` in LazyVim.

---

## License

MIT License. Copyright (c) 2026 The Rost Project Contributors.
