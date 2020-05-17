package mr

import (
	"encoding/json"
	"fmt"
	"hash/fnv"
	"io/ioutil"
	"log"
	"net/rpc"
	"os"
	"sort"
)

// for sorting by key.
type ByKey []KeyValue

// for sorting by key.
func (a ByKey) Len() int           { return len(a) }
func (a ByKey) Swap(i, j int)      { a[i], a[j] = a[j], a[i] }
func (a ByKey) Less(i, j int) bool { return a[i].Key < a[j].Key }

//
// Map functions return a slice of KeyValue.
//
type KeyValue struct {
	Key   string
	Value string
}

//
// use ihash(key) % NReduce to choose the reduce
// task number for each KeyValue emitted by Map.
//
func ihash(key string) int {
	h := fnv.New32a()
	h.Write([]byte(key))
	return int(h.Sum32() & 0x7fffffff)
}

//
// main/mrworker.go calls this function.
//
func Worker(mapf func(string, string) []KeyValue,
	reducef func(string, []string) string) {

	// Your worker implementation here.

	// 实现第一步，尝试从master获取任务
	// map任务全部完成才能开始reduce任务
	args := TaskArgs{}
	for reply := CallMaster(args); reply != nil && len(reply.Filename) != 0; {
		if reply.TaskType == Map {
			mapWork(*reply, mapf)
			// reduceWork(*reply, intermediate, reducef)
			args = TaskArgs{}
			args.LastFileName = reply.Filename
			args.TaskIndex = reply.TaskIndex
			reply = CallMaster(args)
		}
		if reply.TaskType == Reduce {
			reduceWork(*reply, reducef)
		}

	}

	// uncomment to send the Example RPC to the master.
	// RPC 调用示例
	// CallExample()

}

// 每个map任务都会有一个属于自己的编号x，接收一个<filename, content>内容，初始化一个大小为nReduce（reduce的任务数）的桶buckets，
// 读取content内容，每遇到一个单词就生成一个<word, 1>（映射规则），然后根据word与nReduce（reduce的任务数）进行哈希，得到一个桶序号y，
// 并将这个单词放入buckets[y]中，等处理完后，将buckets内容保存至文件中，每个bucket对应一个文件，文件名为mr-x-y
func mapWork(reply TaskReply, mapf func(string, string) []KeyValue) {
	filename := reply.Filename
	bucketCount := reply.ReduceNum
	buckets := make([][]KeyValue, bucketCount)
	file, err := os.Open(filename)
	if err != nil {
		log.Fatalf("cannot open %v", filename)
	}
	content, err := ioutil.ReadAll(file)
	if err != nil {
		log.Fatalf("cannot read %v", filename)
	}
	file.Close()
	kva := mapf(filename, string(content))
	for _, kv := range kva {
		intermediate := ihash(kv.Key) % bucketCount
		bucket := buckets[intermediate]
		buckets[intermediate] = append(bucket, kv)
	}
	for index, bucket := range buckets {
		fileName := "./mr-tmp/mr" + fmt.Sprintf("-%d", reply.TaskIndex) + fmt.Sprintf("-%d", index)
		os.Remove(fileName)
		file, _ := os.Create(fileName)
		enc := json.NewEncoder(file)
		enc.Encode(&bucket)
	}

}

func reduceWork(reply TaskReply, reducef func(string, []string) string) {
	//
	// a big difference from real MapReduce is that all the
	// intermediate data is in one place, intermediate[],
	// rather than being partitioned into NxM buckets.
	//
	taskIndex := reply.TaskIndex
	// var KeyValue intermediate=
	// sort.Sort(ByKey(intermediate))

	oname := "mr-" + fmt.Sprintf("%d", taskIndex)
	log.Println("oname: ", oname)
	ofile, _ := os.Create(oname)

	//
	// call Reduce on each distinct key in intermediate[],
	// and print the result to mr-out-0.
	//
	i := 0
	for i < len(intermediate) {
		j := i + 1
		for j < len(intermediate) && intermediate[j].Key == intermediate[i].Key {
			j++
		}
		values := []string{}
		for k := i; k < j; k++ {
			values = append(values, intermediate[k].Value)
		}
		output := reducef(intermediate[i].Key, values)

		// this is the correct format for each line of Reduce output.
		fmt.Fprintf(ofile, "%v %v\n", intermediate[i].Key, output)

		i = j
	}

	ofile.Close()
}

//
// 创建RPC调用，调用master，获取对应的任务
func CallMaster(args TaskArgs) *TaskReply {
	reply := TaskReply{}
	if call("Master.SendTask", &args, &reply) {
		return &reply
	}
	return nil

}

//
// example function to show how to make an RPC call to the master.
//
// the RPC argument and reply types are defined in rpc.go.
//
func CallExample() {

	// declare an argument structure.
	args := ExampleArgs{}

	// fill in the argument(s).
	args.X = 99

	// declare a reply structure.
	reply := ExampleReply{}

	// send the RPC request, wait for the reply.
	call("Master.Example", &args, &reply)

	// reply.Y should be 100.
	fmt.Printf("reply.Y %v\n", reply.Y)
}

//
// send an RPC request to the master, wait for the response.
// usually returns true.
// returns false if something goes wrong.
//
func call(rpcname string, args interface{}, reply interface{}) bool {
	// c, err := rpc.DialHTTP("tcp", "127.0.0.1"+":1234")
	sockname := masterSock()
	log.Println("worker sock name: ", sockname)
	c, err := rpc.DialHTTP("unix", sockname)
	if err != nil {
		log.Fatal("dialing:", err)
	}
	defer c.Close()

	err = c.Call(rpcname, args, reply)
	if err == nil {
		return true
	}

	fmt.Println(err)
	return false
}
