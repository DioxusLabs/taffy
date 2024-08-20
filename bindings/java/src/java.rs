use jni::objects::{JObject, JValue};
use jni::sys::jobject;
use jni::JNIEnv;
use taffy::{Layout, Point, Rect, Size};

pub fn to_java_layout(env: &mut JNIEnv, layout: &Layout) -> jobject {
    let layout_class = &env.find_class("com/dioxuslabs/taffy/tree/TaffyLayout").unwrap();

    let location = &f32_point_to_java(env, layout.location);
    let size = &f32_size_to_java(env, layout.size);
    let content_size = &f32_size_to_java(env, layout.content_size);
    let scrollbar_size = &f32_size_to_java(env, layout.scrollbar_size);
    let border = &f32_rect_to_java(env, layout.border);
    let padding = &f32_rect_to_java(env, layout.padding);
    let margin = &f32_rect_to_java(env, layout.margin);

    *env.new_object(layout_class, "(ILcom/dioxuslabs/taffy/geom/TaffyPoint;Lcom/dioxuslabs/taffy/geom/TaffySize;Lcom/dioxuslabs/taffy/geom/TaffySize;Lcom/dioxuslabs/taffy/geom/TaffySize;Lcom/dioxuslabs/taffy/geom/TaffyRect;Lcom/dioxuslabs/taffy/geom/TaffyRect;Lcom/dioxuslabs/taffy/geom/TaffyRect;)V", &[
        JValue::Int(layout.order as i32),
        JValue::Object(location),
        JValue::Object(size),
        JValue::Object(content_size),
        JValue::Object(scrollbar_size),
        JValue::Object(border),
        JValue::Object(padding),
        JValue::Object(margin),
    ]).unwrap()
}

pub fn f32_point_to_java<'local>(env: &mut JNIEnv<'local>, point: Point<f32>) -> JObject<'local> {
    let class = &env.find_class("com/dioxuslabs/taffy/geom/TaffyPoint").unwrap();

    let a = &f32_to_java(env, point.x);
    let b = &f32_to_java(env, point.y);

    env.new_object(class, "(Ljava/lang/Object;Ljava/lang/Object;)V", &[JValue::Object(a), JValue::Object(b)]).unwrap()
}

pub fn f32_size_to_java<'local>(env: &mut JNIEnv<'local>, size: Size<f32>) -> JObject<'local> {
    let class = &env.find_class("com/dioxuslabs/taffy/geom/TaffySize").unwrap();

    let a = &f32_to_java(env, size.width);
    let b = &f32_to_java(env, size.height);

    env.new_object(class, "(Ljava/lang/Object;Ljava/lang/Object;)V", &[JValue::Object(a), JValue::Object(b)]).unwrap()
}

pub fn f32_rect_to_java<'local>(env: &mut JNIEnv<'local>, rect: Rect<f32>) -> JObject<'local> {
    let class = &env.find_class("com/dioxuslabs/taffy/geom/TaffyRect").unwrap();

    let a = &f32_to_java(env, rect.left);
    let b = &f32_to_java(env, rect.right);
    let c = &f32_to_java(env, rect.top);
    let d = &f32_to_java(env, rect.bottom);

    env.new_object(
        class,
        "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
        &[JValue::Object(a), JValue::Object(b), JValue::Object(c), JValue::Object(d)],
    )
    .unwrap()
}

pub fn f32_to_java<'local>(env: &mut JNIEnv<'local>, value: f32) -> JObject<'local> {
    let class = &env.find_class("java/lang/Float").unwrap();

    env.new_object(class, "(F)V", &[JValue::Float(value)]).unwrap()
}
