package app.visly.stretch

internal fun Enum<*>.toFloatArray(): FloatArray {
    return floatArrayOf(ordinal.toFloat())
}

internal fun Float?.toFloatArray(): FloatArray {
    return if (this == null) floatArrayOf(0f, 0f) else floatArrayOf(1f, this)
}

enum class AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

enum class AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

enum class AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

enum class Direction {
    Inherit,
    LTR,
    RTL,
}

enum class Display {
    Flex,
    None,
}

enum class FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

enum class JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

enum class Overflow {
    Visible,
    Hidden,
    Scroll,
}

enum class PositionType {
    Relative,
    Absolute,
}

enum class FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

sealed class Dimension {
    data class Points(val points: Float) : Dimension()
    data class Percent(val percentage: Float) : Dimension()
    object Undefined : Dimension()
    object Auto : Dimension()


    internal fun toFloatArray(): FloatArray {
        val type = when (this) {
            is Dimension.Points -> 0f
            is Dimension.Percent -> 1f
            is Dimension.Undefined -> 2f
            is Dimension.Auto -> 3f
        }

        val value = when (this) {
            is Dimension.Points -> this.points
            is Dimension.Percent -> this.percentage
            is Dimension.Undefined -> 0f
            is Dimension.Auto -> 0f
        }

        return floatArrayOf(type, value)
    }
}

data class Size<T>(
    val width: T,
    val height: T)

internal fun Size<Dimension>.toFloatArray(): FloatArray {
    return flatten(
        width.toFloatArray(),
        height.toFloatArray()
    )
}

internal fun Size<Float?>.toFloatArray2(): FloatArray {
    return flatten(
        width.toFloatArray(),
        height.toFloatArray()
    )
}

data class Rect<T>(
    val start: T,
    val end: T,
    val top: T,
    val bottom: T)

internal fun Rect<Dimension>.toFloatArray(): FloatArray {
    return flatten(
        start.toFloatArray(),
        end.toFloatArray(),
        top.toFloatArray(),
        bottom.toFloatArray()
    )
}

data class Style(
    val display: Display = Display.Flex,
    val positionType: PositionType = PositionType.Relative,
    val direction: Direction = Direction.Inherit,
    val flexDirection: FlexDirection = FlexDirection.Row,
    val flexWrap: FlexWrap = FlexWrap.NoWrap,
    val overflow: Overflow = Overflow.Hidden,
    val alignItems: AlignItems = AlignItems.Stretch,
    val alignSelf: AlignSelf = AlignSelf.Auto,
    val alignContent: AlignContent = AlignContent.FlexStart,
    val justifyContent: JustifyContent = JustifyContent.FlexStart,
    val position: Rect<Dimension> = Rect(Dimension.Undefined, Dimension.Undefined, Dimension.Undefined, Dimension.Undefined),
    val margin: Rect<Dimension> = Rect(Dimension.Undefined, Dimension.Undefined, Dimension.Undefined, Dimension.Undefined),
    val padding: Rect<Dimension> = Rect(Dimension.Undefined, Dimension.Undefined, Dimension.Undefined, Dimension.Undefined),
    val border: Rect<Dimension> = Rect(Dimension.Undefined, Dimension.Undefined, Dimension.Undefined, Dimension.Undefined),
    val flexGrow: Float = 0f,
    val flexShrink: Float = 1f,
    val flexBasis: Dimension = Dimension.Auto,
    val size: Size<Dimension> = Size(Dimension.Auto, Dimension.Auto),
    val minSize: Size<Dimension> = Size(Dimension.Auto, Dimension.Auto),
    val maxSize: Size<Dimension> = Size(Dimension.Auto, Dimension.Auto),
    val aspectRatio: Float? = null) {

    internal fun toFloatArray(): FloatArray {
        return flatten(
            display.toFloatArray(),
            positionType.toFloatArray(),
            direction.toFloatArray(),
            flexDirection.toFloatArray(),
            flexWrap.toFloatArray(),
            overflow.toFloatArray(),
            alignItems.toFloatArray(),
            alignSelf.toFloatArray(),
            alignContent.toFloatArray(),
            justifyContent.toFloatArray(),

            position.toFloatArray(),
            margin.toFloatArray(),
            padding.toFloatArray(),
            border.toFloatArray(),

            floatArrayOf(flexGrow, flexShrink),
            flexBasis.toFloatArray(),
            size.toFloatArray(),
            minSize.toFloatArray(),
            maxSize.toFloatArray(),
            aspectRatio.toFloatArray()
        )
    }
}


internal fun flatten(vararg args: FloatArray): FloatArray {
    var len = 0

    for (a in args) {
        len += a.size
    }

    var result = FloatArray(len)
    for (a in args) {
        result += a
    }

    return result
}