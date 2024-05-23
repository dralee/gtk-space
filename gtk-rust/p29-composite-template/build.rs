/**
 * 编译资源文件
 * 2024.05.20 by dralee
 */
fn main(){
    glib_build_tools::compile_resources(&["src/resources"], 
        "src/resources/resources.gresource.xml", 
        "composite_template1.gresource",);
}