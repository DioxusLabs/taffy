import Foundation

public enum AlignItems: Int {
    case flexStart
    case flexEnd
    case center
    case baseline
    case stretch
}

public enum AlignSelf: Int {
    case auto
    case flexStart
    case flexEnd
    case center
    case baseline
    case stretch
}

public enum AlignContent: Int {
    case flexStart
    case flexEnd
    case center
    case stretch
    case spaceBetween
    case spaceAround
}

public enum Direction: Int {
    case inherit
    case ltr
    case rtl
}

public enum Display: Int {
    case flex
    case none
}

public enum FlexDirection: Int {
    case row
    case column
    case rowReverse
    case columnReverse
}

public enum JustifyContent: Int {
    case flexStart
    case flexEnd
    case center
    case spaceBetween
    case spaceAround
    case spaceEvenly
}

public enum Overflow: Int {
    case visible
    case hidden
    case scroll
}

public enum PositionType: Int {
    case relative
    case absolute
}

public enum FlexWrap: Int {
    case noWrap
    case wrap
    case wrapReverse
}

public enum Dimension {
    case points(Float)
    case percent(Float)
    case auto
    case undefined
    
    fileprivate func intoStretchValue() -> StretchStyleDimension {
        switch self {
        case .points(let value): return StretchStyleDimension(dimen_type: 0, dimen_value: value)
        case .percent(let value): return StretchStyleDimension(dimen_type: 1, dimen_value: value)
        case .auto: return StretchStyleDimension(dimen_type: 2, dimen_value: 0.0)
        case .undefined: return StretchStyleDimension(dimen_type: 3, dimen_value: 0.0)
        }
    }
}

public struct Size<T> {
    public let width: T
    public let height: T
    
    public init(width: T, height: T) {
        self.width = width
        self.height = height
    }
}

public struct Rect<T> {
    public let start: T
    public let end: T
    public let top: T
    public let bottom: T
    
    public init(start: T, end: T, top: T, bottom: T) {
        self.start = start
        self.end = end
        self.top = top
        self.bottom = bottom
    }
}

public class Style {
    internal let rustptr: UnsafeMutableRawPointer
    
    public let display: Display
    public let positionType: PositionType
    public let direction: Direction
    public let flexDirection: FlexDirection
    public let flexWrap: FlexWrap
    public let overflow: Overflow
    public let alignItems: AlignItems
    public let alignSelf: AlignSelf
    public let alignContent: AlignContent
    public let justifyContent: JustifyContent
    public let position: Rect<Dimension>
    public let margin: Rect<Dimension>
    public let padding: Rect<Dimension>
    public let border: Rect<Dimension>
    public let flexGrow: Float
    public let flexShrink: Float
    public let flexBasis: Dimension
    public let size: Size<Dimension>
    public let minSize: Size<Dimension>
    public let maxSize: Size<Dimension>
    public let aspectRatio: Float?
    
    public init(
        display: Display = .flex,
        positionType: PositionType = .relative,
        direction: Direction = .inherit,
        flexDirection: FlexDirection = .row,
        flexWrap: FlexWrap = .noWrap,
        overflow: Overflow = .hidden,
        alignItems: AlignItems = .stretch,
        alignSelf: AlignSelf = .auto,
        alignContent: AlignContent = .flexStart,
        justifyContent: JustifyContent = .flexStart,
        position: Rect<Dimension> = Rect(start: .undefined, end: .undefined, top: .undefined, bottom: .undefined),
        margin: Rect<Dimension> = Rect(start: .undefined, end: .undefined, top: .undefined, bottom: .undefined),
        padding: Rect<Dimension> = Rect(start: .undefined, end: .undefined, top: .undefined, bottom: .undefined),
        border: Rect<Dimension> = Rect(start: .undefined, end: .undefined, top: .undefined, bottom: .undefined),
        flexGrow: Float = 0.0,
        flexShrink: Float = 1.0,
        flexBasis: Dimension = .auto,
        size: Size<Dimension> = Size(width: .auto, height: .auto),
        minSize: Size<Dimension> = Size(width: .auto, height: .auto),
        maxSize: Size<Dimension> = Size(width: .auto, height: .auto),
        aspectRatio: Float? = nil
    ) {
        self.display = display
        self.positionType = positionType
        self.direction = direction
        self.flexDirection = flexDirection
        self.flexWrap = flexWrap
        self.overflow = overflow
        self.alignItems = alignItems
        self.alignSelf = alignSelf
        self.alignContent = alignContent
        self.justifyContent = justifyContent
        self.position = position
        self.margin = margin
        self.padding = padding
        self.border = border
        self.flexGrow = flexGrow
        self.flexShrink = flexShrink
        self.flexBasis = flexBasis
        self.size = size
        self.minSize = minSize
        self.maxSize = maxSize
        self.aspectRatio = aspectRatio
        
        self.rustptr = stretch_style_create(
            Int32(display.rawValue),
            Int32(positionType.rawValue),
            Int32(direction.rawValue),
            Int32(flexDirection.rawValue),
            Int32(flexWrap.rawValue),
            Int32(overflow.rawValue),
            Int32(alignItems.rawValue),
            Int32(alignSelf.rawValue),
            Int32(alignContent.rawValue),
            Int32(justifyContent.rawValue),
            StretchStyleRect(
                start: position.start.intoStretchValue(),
                end: position.end.intoStretchValue(),
                top: position.top.intoStretchValue(),
                bottom: position.bottom.intoStretchValue()
            ),
            StretchStyleRect(
                start: margin.start.intoStretchValue(),
                end: margin.end.intoStretchValue(),
                top: margin.top.intoStretchValue(),
                bottom: margin.bottom.intoStretchValue()
            ),
            StretchStyleRect(
                start: padding.start.intoStretchValue(),
                end: padding.end.intoStretchValue(),
                top: padding.top.intoStretchValue(),
                bottom: padding.bottom.intoStretchValue()
            ),
            StretchStyleRect(
                start: border.start.intoStretchValue(),
                end: border.end.intoStretchValue(),
                top: border.top.intoStretchValue(),
                bottom: border.bottom.intoStretchValue()
            ),
            flexGrow,
            flexShrink,
            flexBasis.intoStretchValue(),
            StretchStyleSize(width: size.width.intoStretchValue(), height: size.height.intoStretchValue()),
            StretchStyleSize(width: minSize.width.intoStretchValue(), height: minSize.height.intoStretchValue()),
            StretchStyleSize(width: maxSize.width.intoStretchValue(), height: maxSize.height.intoStretchValue()),
            aspectRatio ?? Float.nan)
    }
    
    deinit {
        stretch_style_free(rustptr)
    }
}
