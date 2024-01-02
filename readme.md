# node_version

`node_version` 是一个用于协助创建临时 Node 环境的 npm 包。

## 安装

使用 npm 进行全局安装：

```
npm install -g node_version
```

## 命令行使用

### 生成临时环境

使用以下命令生成一个临时的 Node 环境：

```
node_version gen -v <node_version> -o <output_file>
```

例如：

```
node_version gen -v 18.18 -o env.sh
```

这将生成一个名为 `env.sh` 的文件，其中包含了使用指定的 Node 版本（18.18）的环境配置。

### 运行命令

使用以下命令在临时环境中运行命令：

```
node_version -v <node_version> --run "<command>"
```

例如：

```
node_version -v 18.18 --run "node -v"
```

这将在使用指定的 Node 版本（18.18）的临时环境中运行 `node -v` 命令，并输出 Node 版本信息。

## 许可证

本项目基于 [MIT 许可证](https://opensource.org/licenses/MIT) 进行授权。

---

希望这份 README 文件对你有所帮助！如有任何问题，请随时提问。