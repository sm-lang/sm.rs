在 projects/sm-runner 目录下, 基于 rust 的 egg(e-graph) 库, 写一个类似 mathematica 的工具, 

支持 模式匹配, replace, expand, simplify, 求导 等基本功能. 只需基于 ast, 无需考虑 parser.


基本功能:
符号支持 namespace, scope
支持常量: x=true, let x=false, null, -1, 0, 1 使用 bigint
支持定义函数 f(x) = 2x^2, def f(x: any) { 2x^2 }
支持定义 rank-n 函数: f(x)(y) = x + y
支持自定义 operator, 请勿添加 Add, Mul 等 ast 节点