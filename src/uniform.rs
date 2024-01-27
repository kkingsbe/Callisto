#[derive(Clone)]
pub enum UniformValue {
    Float(f64),
    Int(i64)
}

#[derive(Clone)]
pub struct Uniform {
    value: UniformValue,
    key: String
}

impl Uniform {
    pub fn new(key: String, value: UniformValue) -> Self {
        Self {
            value,
            key
        }
    }

    pub fn get_value(&self) -> UniformValue {
        self.value.clone()
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }
}

#[derive(Clone)]
pub struct UniformManager {
    pub uniforms: Vec<Uniform>
}

impl UniformManager {
    pub fn new() -> Self {
        Self {
            uniforms: Vec::new()
        }
    }

    pub fn add(&mut self, key: String, value: UniformValue) {
        self.uniforms.push(Uniform { value, key });
    }

    pub fn update_value(&mut self, key: String, new_value: UniformValue) {
        for uniform in &mut self.uniforms {
            if uniform.key == key {
                uniform.value = new_value.clone();
            }
        }
    }

    pub fn get_value(&self, key: &String) -> UniformValue {
        for uniform in &self.uniforms {
            if &uniform.key == key {
                return uniform.value.clone();
            }
        }

        panic!("Uniform {} not found", key);
    }
}