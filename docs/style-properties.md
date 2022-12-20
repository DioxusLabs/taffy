# Style Properties

Y = Supported in spec and implemented in Taffy
~Y = Implemented in Taffy, but not thoroughly tested
N = Supported in spec but not implemented in Taffy
\- = Not applicable to layout mode
1-5 = Priorities for a phased implementation of CSS Grid

| Property                 | Flex | Grid | Description                                                                                 |
| ---                      | ---  | ---  | ---                                                                                         |
| **Layout Mode**          |      |      |                                                                                             |
| `display`                | Y    | Y    | What layout strategy should be used?                                                        |
| **Position**             |      |      |                                                                                             |
| `position_type`          | Y    | 2    | Absolute vs. in-flow position                                                               |
| `position`               | Y    | 2    | How should the position of this element be tweaked relative to the layout defined?          |
| **Item size**            |      |      |                                                                                             |
| `size`                   | Y    | Y    | The nominal height and width of item                                                        |
| `min_size`               | Y    | Y    | The minimum height and width of the item                                                    |
| `max_size`               | Y    | Y    | The maximum height and width of the item                                                    |
| `aspect_ratio`           | Y    | 3    | The preferred aspect ratio (calculated as width divided by height)                          |
| **Item spacing**         |      |      |                                                                                             |
| `padding`                | Y    | ~Y   | How large should the padding be on each side?                                               |
| `border`                 | Y    | ~Y   | How large should the border be on each side?                                                |
| `margin`                 | Y    | ~Y   | How large should the margin be on each side?                                                |
| `gap`                    | Y    | Y    | The size of the vertical and horizontal gaps between flex items / grid rows                 |
| **Alignment**            |      |      |                                                                                             |
| `align_items`            | Y    | Y    | How should items be aligned relative to the cross axis?                                     |
| `align_self`             | Y    | Y    | Should this item violate the cross axis alignment specified by its parent's [`AlignItems`]? |
| `align_content`          | Y    | Y    | How should content contained within this item be aligned relative to the cross axis?        |
| `justify_items`          | -    | Y    | How should items be aligned relative to the main axis?                                      |
| `justify_self`           | -    | Y    | Should this item violate the main axis alignment specified by its parent's [`AlignItems`]?  |
| `justify_content`        | Y    | Y    | How should content contained within this item be aligned relative to the main axis?         |
| **Flexbox**              |      |      |                                                                                             |
| `flex_direction`         | Y    | -    | Which direction does the main axis flow in?                                                 |
| `flex_wrap`              | Y    | -    | Should elements wrap, or stay in a single line?                                             |
| `flex_basis`             | Y    | -    | Sets the initial main axis size of the item                                                 |
| `flex_grow`              | Y    | -    | The relative rate at which this item grows when it is expanding to fill space               |
| `flex_shrink`            | Y    | -    | The relative rate at which this item shrinks when it is contracting to fit into space       |
| **CSS Grid (Container)** |      |      |                                                                                             |
| `grid-template-columns`  | -    | Y    | The track sizing functions of the grid's explicit columns                                   |
| `grid-template-rows`     | -    | Y    | The track sizing functions of the grid's explicit rows                                      |
| `grid-template-areas`    | -    | 5    | Defines named grid areas                                                                    |
| `grid-auto-rows`         | -    | Y    | Track sizing functions for the grid's implicitly generated rows                             |
| `grid-auto-columns`      | -    | Y    | Track sizing functions for the grid's implicitly generated columns                          |
| `grid-auto-flow`         | -    | Y    | Whether auto-placed items are placed row-wise or column-wise. And sparsely or densely.      |
| **CSS Grid (Child)**     |      |      |                                                                                             |
| `grid-row-start`         | -    | Y    | The (row) grid line the item starts at (or a span)                                          |
| `grid-row-end`           | -    | Y    | The (row) grid line the item ends at (or a span)                                            |
| `grid-column-start`      | -    | Y    | The (column) grid line the item starts at (or a span)                                       |
| `grid-column-end`        | -    | Y    | The (column) grid line the item end at (or a span)                                          |
| `grid-column`            | -    | 4    | Shorthand for grid-column-start/grid-column-end                                             |
| `grid-row`               | -    | 4    | Shorthand for grid-row-start/grid-row-end                                                   |
| `grid-area`              | -    | 5    | Accepts either shorthand row/column-start/end or a named grid area                          |
