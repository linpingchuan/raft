# 6.824的工作文档

## map-reduce 的流程文档
### shell 构建脚本
```shell
cd src/main
go build -buildmode=plugin ../mrapps/wc.go
rm mr-out*
go run mrsequential.go wc.go pg*.txt
more mr-out-0
```

### 作业的测试脚本
```shell
go build -buildmode=plugin ../mrapps/wc.go
rm mr-out*
go run mrmaster.go pg-*.txt
go run mrworker.go wc.so
cat mr-out-* | sort | more
```

### 作业全自动化测试脚本
```shell
cd src/main
sh test-mr.sh
```