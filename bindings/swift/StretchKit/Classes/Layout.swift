import Foundation

public class Layout {
    public let x: Float
    public let y: Float
    public let width: Float
    public let height: Float
    public let children: Array<Layout>
    
    internal init(x: Float, y: Float, width: Float, height: Float, children: Array<Layout>) {
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.children = children
    }
    
    internal static func fromFloats(_ floats: UnsafePointer<Float>) -> (UnsafePointer<Float>, Layout) {
        let x = floats.pointee;
        var floats = floats.advanced(by: 1)
        
        let y = floats.pointee
        floats = floats.advanced(by: 1)
        
        let width = floats.pointee
        floats = floats.advanced(by: 1)
        
        let height = floats.pointee
        floats = floats.advanced(by: 1)
        
        let childCount = Int(floats.pointee)
        floats = floats.advanced(by: 1)
        
        var children = [Layout]()
        
        for _ in 0..<childCount {
            let child = Layout.fromFloats(floats)
            floats = child.0
            children.append(child.1)
        }
        
        return (floats, Layout(x: x, y: y, width: width, height: height, children: children))
    }
}
