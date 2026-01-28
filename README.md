# doc-to-pdf

一个可以将Word、Excel、PPT、CSV文件转换为PDF的CLI工具。

嵌入了精简后的 `libreoffice`。

支持的文件格式

- Word: `.doc`, `.docx`
- Excel: `.xls`, `.xlsx`
- PPT: `.ppt`, `.pptx`
- 其他: `.csv`


## 使用方法

编译：
```bash
git clone https://github.com/xgpxg/doc-to-pdf.git

cd doc-to-pdf

cargo build -r
```

```bash
# 基本用法
./doc-to-pdf -i input.docx -o output.pdf

# 或者使用Cargo运行
cargo run -- -i input.docx -o output.pdf
```

参数说明：
- `-i` 或 `--input`: 输入的源文件路径
- `-o` 或 `--output`: 输出的PDF文件路径
