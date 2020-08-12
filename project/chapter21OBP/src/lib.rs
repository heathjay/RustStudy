pub struct AverageCollection{
    //内部字段是私有的
    list:Vec<i32>,
    average:f64,
}

impl AverageCollection{
    pub fn add(&mut self, value:i32){
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32>{
        let result = self.list.pop();
        match result{
            Some(value) =>{
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }
    pub fn average(&self) -> f64{
        self.average
    }
    fn update_average(&mut self){
        let total : i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

//gui设计
pub trait Draw{
    fn draw(&self);
}
//Box<dyn Draw>作为一个trait对象：他时box中任何实现了draw trait的类型的替身
pub struct Screen{
    pub components:Vec<Box<dyn Draw>>,
}
impl Screen{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}
/**
 * 泛型和trait bound
 */
/*
pub struct Screen<T: Draw>{
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T:Draw{
        pub fn run(&self){
            for component in self.components.iter(){
                component.draw();
            }
        }
    }
*/

//实现trait
pub struct Button{
    pub width: u32,
    pub height: u32,
    pub label : String,
}

impl Draw for Button{
    fn draw(&self){
        //实现代码
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
