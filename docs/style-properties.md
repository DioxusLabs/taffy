# Style Properties

Y = Supported in spec and implemented in Taffy
N = Supported in spec but not implemented in Taffy
\- = Not applicable to layout mode
1-5 = Priorities for a phased implementation of CSS Grid

| Property                 | Flex | Grid | Description                                                                                 |
| ---                      | ---  | ---  | ---                                                                                         |
| **Layout Mode**          |      |      |                                                                                             |
| `display`                | Y    | 1    | What layout strategy should be used?                                                        |
| **Position**             |      |      |                                                                                             |
| `position_type`          | Y    | 2    | Absolute vs. in-flow position                                                               |
| `position`               | Y    | 2    | How should the position of this element be tweaked relative to the layout defined?          |
| **Item size**            |      |      |                                                                                             |
| `size`                   | Y    | 1    | The nominal height and width of item                                                        |
| `min_size`               | Y    | 1    | The minimum height and width of the item                                                    |
| `max_size`               | Y    | 1    | The maximum height and width of the item                                                    |
| `aspect_ratio`           | Y    | 3    | The preferred aspect ratio (calculated as width divided by height)                          |
| `padding`                | Y    | 1    | How large should the padding be on each side?                                               |
| `border`                 | Y    | 1    | How large should the border be on each side?                                                |
| **Item spacing**         |      |      |                                                                                             |
| `margin`                 | Y    | 1    | How large should the margin be on each side?                                                |
| `gap`                    | N    | 2    | The size of the vertical and horizontal gaps between flex items / grid rows                 |
| **Alignment**            |      |      |                                                                                             |
| `align_items`            | Y    | 3    | How should items be aligned relative to the cross axis?                                     |
| `align_self`             | Y    | 3    | Should this item violate the cross axis alignment specified by its parent's [`AlignItems`]? |
| `align_content`          | Y    | 3    | How should content contained within this item be aligned relative to the cross axis?        |
| `justify_items`          | N    | 3    | How should items be aligned relative to the main axis?                                      |
| `justify_self`           | N    | 3    | Should this item violate the main axis alignment specified by its parent's [`AlignItems`]?  |
| `justify_content`        | Y    | 3    | How should content contained within this item be aligned relative to the main axis?         |
| **Flexbox**              |      |      |                                                                                             |
| `flex_direction`         | Y    | -    | Which direction does the main axis flow in?                                                 |
| `flex_wrap`              | Y    | -    | Should elements wrap, or stay in a single line?                                             |
| `flex_basis`             | Y    | -    | Sets the initial main axis size of the item                                                 |
| `flex_grow`              | Y    | -    | The relative rate at which this item grows when it is expanding to fill space               |
| `flex_shrink`            | Y    | -    | The relative rate at which this item shrinks when it is contracting to fit into space       |
| **CSS Grid (Container)** |      |      |                                                                                             |
| `grid-template-columns`  | -    | 1    |                                                                                             |
| `grid-template-rows`     | -    | 1    |                                                                                             |
| `grid-template-areas`    | -    | 5    |                                                                                             |
| `grid-auto-rows`         | -    | 1    |                                                                                             |
| `grid-auto-columns`      | -    | 1    |                                                                                             |
| `grid-auto-flow`         | -    | 1    |                                                                                             |
| **CSS Grid (Child)**     |      |      |                                                                                             |
| `grid-row-start`         | -    | 1    |                                                                                             |
| `grid-row-end`           | -    | 1    |                                                                                             |
| `grid-column-start`      | -    | 1    |                                                                                             |
| `grid-column-end`        | -    | 1    |                                                                                             |
| `grid-column`            | -    | 1    |                                                                                             |
| `grid-row`               | -    | 1    |                                                                                             |
| `grid-area`              | -    | 1    |                                                                                             |
