use std::boxed::Box;
use std::fs;
use std::fs::File;

// 定义 Node 枚举
#[derive(Debug)]
enum Node {
    File(String),
    Folder(Box<FolderNode>),
}

// 定义 FileSystem Trait
trait FileSystem {
    fn create_file(&mut self, name: String) -> Result<(), String>;
    fn create_folder(&mut self, name: String) -> Result<(), String>;
    fn list_contents(&self) -> String;
}

// 定义 FolderNode 结构体
#[derive(Debug)]
struct FolderNode {
    name: String,
    contents: Vec<Box<Node>>,
}

impl FolderNode {
    fn new(name: String) -> Self {
        FolderNode {
            name,
            contents: Vec::new(),
        }
    }
}

impl FileSystem for FolderNode {
    //创建文件
    fn create_file(&mut self, name: String) -> Result<(), String> {
        if self.contents.iter().any(|node| match node.as_ref() {
            Node::File(f) if f == &name => true,
            _ => false,
        }) {
            return Err(format!("File '{}' already exists.", name));
        }
        match File::create(name.clone()) {
            Ok(_) => println!("文件 已成功创建"),
            Err(e) => {
                eprintln!("创建文件时发生错误: {}", e);
            }
        }
        self.contents.push(Box::new(Node::File(name)));
        Ok(())
    }
    //创建文件夹
    fn create_folder(&mut self, name: String) -> Result<(), String> {
        if self.contents.iter().any(|node| match node.as_ref() {
            Node::Folder(f) if f.name == name => true,
            _ => false,
        }) {
            return Err(format!("Folder '{}' already exists.", name));
        }
        match fs::create_dir_all(name.clone()) {
            Ok(_) => println!("文件夹 已成功创建"),
            Err(e) => {
                eprintln!("创建文件夹时发生错误: {}", e);
                // 你可以在这里处理错误，比如重试或退出程序
            }
        }
        self.contents
            .push(Box::new(Node::Folder(Box::new(FolderNode::new(name)))));
        Ok(())
    }
    //查询文件结构
    fn list_contents(&self) -> String {
        let mut result = String::new();
        for node in &self.contents {
            match node.as_ref() {
                Node::File(name) => {
                    result += &format!("File: {}\n", name);
                }
                Node::Folder(folder) => {
                    result += &format!("Folder: {}\n", folder.name);
                    result += &folder.list_contents();
                }
            }
        }
        result
    }
}

// 为 FolderNode 添加一个方法，使其能调用 list_contents，因为 list_contents 是 FileSystem trait 的一部分
impl FolderNode {
    fn list_contents_impl(&self) -> String {
        self.list_contents()
    }
}

fn main() {
    let docFolder = "D:\\rust\\test-rust\\greeting\\box";
    let mut root: FolderNode = FolderNode::new("root".to_string());
    root.create_folder(docFolder.to_string()).unwrap();
    root.create_file(docFolder.to_string() + &"\\README.md".to_string())
        .unwrap();

    println!("{}", root.list_contents_impl());
}
