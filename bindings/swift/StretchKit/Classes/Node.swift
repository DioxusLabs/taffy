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

public class Node {
    private let rustptr: UnsafeMutableRawPointer
    private var style: Style
    private var children: Array<Node> = []
    internal var measure: MeasureFunc? = nil
    
    public init(style: Style, measure: @escaping MeasureFunc) {
        self.rustptr = stretch_node_create(style.rustptr)
        self.style = style
        setMeasure(measure: measure)
    }
    
    public init(style: Style, children: Array<Node>) {
        self.rustptr = stretch_node_create(style.rustptr)
        self.style = style
        setChildren(children: children)
    }
    
    public func setMeasure(measure: @escaping MeasureFunc) {
        stretch_node_set_measure(rustptr, Unmanaged.passRetained(self).toOpaque(), node_measure)
        self.measure = measure;
    }
    
    public func getChildren() -> Array<Node> {
        return self.children
    }
    
    public func setChildren(children: Array<Node>) {
        let num_children = self.children.count
        for i in 0..<num_children {
            removeChildAtIndex(index: num_children - 1 - i)
        }
        
        for child in children {
            addChild(child: child)
        }
    }
    
    public func addChild(child: Node) {
        stretch_node_add_child(rustptr, child.rustptr)
        children.append(child)
    }
    
    @discardableResult
    public func replaceChildAtIndex(index: Int, child: Node) -> Node {
        stretch_node_replace_child_at_index(rustptr, UInt(index), child.rustptr)
        let oldChild = children[index]
        children[index] = child
        return oldChild
    }
    
    @discardableResult
    public func removeChild(child: Node) -> Node {
        stretch_node_remove_child(rustptr, child.rustptr)
        children.removeAll { (node) -> Bool in
            return node === child
        }
        return child
    }
    
    @discardableResult
    public func removeChildAtIndex(index: Int) -> Node {
        stretch_node_remove_child_at_index(rustptr, UInt(index))
        return children.remove(at: index)
    }
    
    public func getStyle() -> Style {
        return self.style
    }
    
    public func setStyle(style: Style) {
        stretch_node_set_style(rustptr, style.rustptr)
        self.style = style
    }
    
    public func markDirty() {
        stretch_node_mark_dirty(rustptr)
    }
    
    public func isDirty() -> Bool {
        return stretch_node_dirty(rustptr)
    }
    
    public func getChildCount() -> Int {
        return self.children.count
    }
    
    public func computeLayout(size: Size<Float?>) -> Layout {
        let layoutPtr = stretch_node_compute_layout(
            rustptr, size.width ?? Float.nan,
            size.height ?? Float.nan,
            create_layout)
        let layout: Layout = Unmanaged.fromOpaque(layoutPtr!).takeUnretainedValue()
        return layout
    }
}
