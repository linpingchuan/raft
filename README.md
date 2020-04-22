# 6.824的工作文档

> git 仓库地址: git://g.csail.mit.edu/6.824-golabs-2020

## map-reduce 的流程文档
> 对应的作业地址: https://pdos.csail.mit.edu/6.824/labs/lab-mr.html

### shell 构建脚本
```shell
cd src/main
go build -buildmode=plugin ../mrapps/wc.go
rm mr-out*
go run mrsequential.go wc.so pg*.txt
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

### 作业全自动化测试脚本通过规范
```shell 
sh ./test-mr.sh
*** Starting wc test.
--- wc test: PASS
*** Starting indexer test.
--- indexer test: PASS
*** Starting map parallelism test.
--- map parallelism test: PASS
*** Starting reduce parallelism test.
--- reduce parallelism test: PASS
*** Starting crash test.
--- crash test: PASS
*** PASSED ALL TESTS
```

## map-reduce规则
1. map阶段将输入的文件分割多个部分的中间值给到**nReduce**个reduce任务，**nReduce**参数将由**main/mrmaster.go**的**MakeMaster()**方法进行传递进去，默认是10个;
2. 每个**mr-out-X**文件每一行都应为每个Reduce函数的输出结果。
每行的格式都应该为 **%v %v** 的键值形式。正确格式在 **main/mrsequential.go** 中，