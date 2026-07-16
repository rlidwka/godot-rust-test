use godot::{
    classes::{
        Control, IControl, Label,
    },
    prelude::*,
};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(tool, base=Control)]
struct HelloWorldTitle {
    base: Base<Control>,

    #[export]
    #[var(set = set_test)]
    test: GString,

    label: Option<Gd<Label>>,
}



#[godot_api]
impl IControl for HelloWorldTitle {
    fn init(base: Base<Control>) -> Self {
        Self {
            base,
            test: GString::from("Hello World"),
            label: None,
        }
    }

    fn ready(&mut self) {
        let mut label = Label::new_alloc();
        label.set_text(&self.test);
        self.label = Some(label.clone());
        self.base_mut().add_child(label.upcast::<Node>().to_godot());
        godot_print!("READY!");
    }
}

#[godot_api]
impl HelloWorldTitle {
    #[func]
    fn set_test(&mut self, value: GString) {
        self.test = value;
        if let Some(label) = &mut self.label {
            label.set_text(&self.test);
        }
    }
}
