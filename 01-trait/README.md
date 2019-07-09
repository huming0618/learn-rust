### Trait总结
- trait有`self`的函数叫asscoiated function，没有的叫叫static function，用::来调用
- Self是类型, &self为变量
- `&self` 是 `Self: &self`的简写
- circle.rs总的`impl Cirlcle`为匿名trait,用来定义`inherent methods (内在方法)

### 延神阅读
- https://blog.rust-lang.org/2015/05/11/traits.html
