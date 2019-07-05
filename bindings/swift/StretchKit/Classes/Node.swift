import Foundation

private func node_measure(_ node: UnsafeRawPointer?, _ width: Float, _ height: Float) -> StretchSize {
    let node: Node = Unmanaged.fromOpaque(node!).takeUnretainedValue()
    let size = node.measure!(Size(width: width.isNaN ? nil : width, height: height.isNaN ? nil : height))
    return StretchSize(width: size.width, height: size.height)
}

private func create_layout(_ floats: UnsafePointer<Float>?) -> UnsafeMutableRawPointer? {
    let layout = Layout.fromFloats(floats!).1
    return Unmanaged.passRetained(layout).toOpaque()
}

public typealias MeasureFunc = (Size<Float?>) -> Size<Float>

fileprivate let stretchptr = stretch_init()

public class Node {
    private let rustptr: UnsafeMutableRawPointer
    private var _children: Array<Node> = []
    private var _measure: MeasureFunc? = nil
    
    public var style: Style {
        willSet(style) {
            stretch_node_set_style(stretchptr, rustptr, style.rustptr)
        }
    }
    
    public var children: Array<Node> {
        get {
            return self._children
        }
        set {
            let num_children = _children.count
            for i in 0..<num_children {
                removeChild(at: num_children - 1 - i)
            }
            
            for child in newValue {
                addChild(child)
            }
        }
    }
    
    public var measure: MeasureFunc? {
        get {
            return self._measure
        }
        set {
            self._measure = newValue
            stretch_node_set_measure(stretchptr, rustptr, Unmanaged.passRetained(self).toOpaque(), node_measure)
        }
    }
    
    public init(style: Style, measure: @escaping MeasureFunc) {
        self.rustptr = stretch_node_create(stretchptr, style.rustptr)
        self.style = style
        self.measure = measure
    }
    
    public init(style: Style, children: Array<Node>) {
        self.rustptr = stretch_node_create(stretchptr, style.rustptr)
        self.style = style
        self.children = children
    }
    
    deinit {
        stretch_node_free(stretchptr, rustptr)
    }
    
    public func addChild(_ child: Node) {
        stretch_node_add_child(stretchptr, rustptr, child.rustptr)
        _children.append(child)
    }
    
    @discardableResult
    public func replaceChild(_ child: Node, at index: Int) -> Node {
        stretch_node_replace_child_at_index(stretchptr, rustptr, UInt(index), child.rustptr)
        let oldChild = _children[index]
        _children[index] = child
        return oldChild
    }
    
    @discardableResult
    public func removeChild(_ child: Node) -> Node {
        stretch_node_remove_child(stretchptr, rustptr, child.rustptr)
        _children.removeAll { (node) -> Bool in
            return node === child
        }
        return child
    }
    
    @discardableResult
    public func removeChild(at index: Int) -> Node {
        stretch_node_remove_child_at_index(stretchptr, rustptr, UInt(index))
        return _children.remove(at: index)
    }
    
    public func markDirty() {
        stretch_node_mark_dirty(stretchptr, rustptr)
    }
    
    public var dirty: Bool {
        return stretch_node_dirty(stretchptr, rustptr)
    }
    
    public func computeLayout(thatFits size: Size<Float?>) -> Layout {
        let layoutPtr = stretch_node_compute_layout(
            stretchptr,
            rustptr,
            size.width ?? Float.nan,
            size.height ?? Float.nan,
            create_layout)
        let layout: Layout = Unmanaged.fromOpaque(layoutPtr!).takeUnretainedValue()
        return layout
    }
}
