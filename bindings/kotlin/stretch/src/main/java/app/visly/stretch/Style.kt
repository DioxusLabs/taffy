package app.visly.stretch

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

    internal val type: Int
        get() = when (this) {
            is Points -> 0
            is Percent -> 1
            is Undefined -> 2
            is Auto -> 3
        }

    internal val value: Float
        get() = when (this) {
            is Points -> this.points
            is Percent -> this.percentage
            is Undefined -> 0f
            is Auto -> 0f
        }
}

data class FloatSize(
    val width: Float,
    val height: Float)

data class Size<T>(
    val width: T,
    val height: T)

data class Rect<T>(
    val start: T,
    val end: T,
    val top: T,
    val bottom: T)

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

    companion object {
        init {
            System.loadLibrary("stretch")
        }
    }

    internal val rustptr: Long

    init {
        rustptr = nConstruct(
            display.ordinal,
            positionType.ordinal,
            direction.ordinal,
            flexDirection.ordinal,
            flexWrap.ordinal,
            overflow.ordinal,
            alignItems.ordinal,
            alignSelf.ordinal,
            alignContent.ordinal,
            justifyContent.ordinal,

            position.start.type,
            position.start.value,
            position.end.type,
            position.end.value,
            position.top.type,
            position.top.value,
            position.bottom.type,
            position.bottom.value,

            margin.start.type,
            margin.start.value,
            margin.end.type,
            margin.end.value,
            margin.top.type,
            margin.top.value,
            margin.bottom.type,
            margin.bottom.value,

            padding.start.type,
            padding.start.value,
            padding.end.type,
            padding.end.value,
            padding.top.type,
            padding.top.value,
            padding.bottom.type,
            padding.bottom.value,

            border.start.type,
            border.start.value,
            border.end.type,
            border.end.value,
            border.top.type,
            border.top.value,
            border.bottom.type,
            border.bottom.value,

            flexGrow,
            flexShrink,

            flexBasis.type,
            flexBasis.value,

            size.width.type,
            size.width.value,
            size.height.type,
            size.height.value,

            minSize.width.type,
            minSize.width.value,
            minSize.height.type,
            minSize.height.value,

            maxSize.width.type,
            maxSize.width.value,
            maxSize.height.type,
            maxSize.height.value,

            aspectRatio ?: Float.NaN
        )
    }

    protected fun finalize() {
        nFree(rustptr)
    }

    private external fun nFree(ptr: Long)

    private external fun nConstruct(
        display: Int,
        positionType: Int,
        direction: Int,
        flexDirection: Int,
        flexWrap: Int,
        overflow: Int,
        alignItems: Int,
        alignSelf: Int,
        alignContent: Int,
        justifyContent: Int,

        positionStartType: Int,
        positionStartValue: Float,
        positionEndType: Int,
        positionEndValue: Float,
        positionTopType: Int,
        positionTopValue: Float,
        positionBottomType: Int,
        positionBottomValue: Float,

        marginStartType: Int,
        marginStartValue: Float,
        marginEndType: Int,
        marginEndValue: Float,
        marginTopType: Int,
        marginTopValue: Float,
        marginBottomType: Int,
        marginBottomValue: Float,

        paddingStartType: Int,
        paddingStartValue: Float,
        paddingEndType: Int,
        paddingEndValue: Float,
        paddingTopType: Int,
        paddingTopValue: Float,
        paddingBottomType: Int,
        paddingBottomValue: Float,

        borderStartType: Int,
        borderStartValue: Float,
        borderEndType: Int,
        borderEndValue: Float,
        borderTopType: Int,
        borderTopValue: Float,
        borderBottomType: Int,
        borderBottomValue: Float,

        flexGrow: Float,
        flexShrink: Float,

        flexBasisType: Int,
        flexBasisValue: Float,

        widthType: Int,
        widthValue: Float,
        heightType: Int,
        heightValue: Float,

        minWidthType: Int,
        minWidthValue: Float,
        minHeightType: Int,
        minHeightValue: Float,

        maxWidthType: Int,
        maxWidthValue: Float,
        maxHeightType: Int,
        maxHeightValue: Float,

        aspectRatio: Float
    ): Long
}
