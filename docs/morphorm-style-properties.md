# Morphorm Styles

## Unique Types

`LayoutType` (corresponds to `Display` in Taffy):

```rust
enum LayoutType {
    Row,
    Column,
    Grid,
}
```

`PositionType` (corresponds to `Position` in Taffy):

```rust
pub enum PositionType {
    SelfDirected, // = Position::Absolute
    ParentDirected, // = Position::Relative
}
```

`Units` (corresponds to `Dimension` in Taffy):

```rust
pub enum Units {
    Pixels(f32), // = Dimension::Points
    Percentage(f32), // = Dimension::Percent
    Auto, // = Dimension::Auto
    Stretch(f32), // No equivalent in Taffy!
}
```

## Style Properties

| Property               | Type          | Description                                                         |
| ---                    | ---           | ---                                                                 |
| **Layout Mode**        |               |                                                                     |
| `layout_mode`          | `LayoutType`  | Row vs. Column vs. Grid                                             |
| `position_type`        | `Position`    | SelfDirected (absolute) vs. ParentDirected (in-flow) position       |
| **Item size**          |               |                                                                     |
| `size`                 | `Size<Units>` | The preferred height and width of item                              |
| `min_size`             | `Size<Units>` | The minimum height and width of the item                            |
| `max_size`             | `Size<Units>` | The maximum height and width of the item                            |
| **Border**             |               |                                                                     |
| `border`               | `Rect<Units>` | How large should the border be on each side?                        |
| **Morphorm Container** |               |                                                                     |
| `child_spacing`        | `Rect<Units>` | Sets the default "spacing" (~margin) on each side of child nodes    |
| `row_between`          | `Units`       | Sets the default vertical "spacing" (~margin) between child nodes   |
| `col_between`          | `Units`       | Sets the default horizontal "spacing" (~margin) between child nodes |
| `grid_rows`            | `Vec<Units>`  | (Grid Container) Row definitions with a size for each row           |
| `grid_cols`            | `Vec<Units>`  | (Grid Container) Column definitions with a size for each column     |
| **Morphorm Item**      |               |                                                                     |
| `spacing`              | `Rect<Units>` | The preferred spacing on each side of the item                      |
| `min_spacing`          | `Rect<Units>` | The minimum spacing on each side of the item                        |
| `max_spacing`          | `Rect<Units>` | The maximum spacing on each side of the item                        |
| `row_index`            | `usize`       | (Grid Item) Zero-based index for the start row of the item          |
| `col_index`            | `usize`       | (Grid Item) Zero-based index for the start column of the item       |
| `row_span`             | `usize`       | (Grid Item) The number of rows the item spans                       |
| `col_span`             | `usize`       | (Grid Item) The number of columns the item spans                    |

### Analysis

Nearly everything in Morphorm is simplified version of either Flexbox or CSS Grid. There are really three things that aren't:

- The `child_spacing` and `*_between` properties. CSS does not allow you to specify child margins on the parent like this. It does have the `align-items` and `justify-items` properties, but those are not as powerful.
- The `min_spacing` and `max_spacing` properties. CSS does not allow you to specify min or max values for margins.
- The `Stretch` variant of `Units`. Flexbox/CSS Grid do not allow you to express "fill available space" as a size like this. To achieve this with Flexbox/CSS Grid you need to use special properties that are specified separately.

The only really tricky thing here is the `width`/`height` properties. They are common to all algorithms, and furthermore by *both* parent and child nodes access this property. This means that the type of these properties really needs to be a single unified type. I thought this might be a blocker, however I have discovered that an upcoming CSS standard ([[css-sizing-4](https://www.w3.org/TR/css-sizing-4)]) actually does [define this functionality](https://www.w3.org/TR/css-sizing-4/#stretch-fit-sizing)) and in fact even calls it `stretch`. The CSS version doesn't have the weighting parameter (although I have [opened an issue](https://github.com/w3c/csswg-drafts/issues/8267) proposing that it does - no idea if that's the right place to do that though) so it is roughly equivalent to `Stretch(1.0)`, but that would a pretty straightforward extension.
