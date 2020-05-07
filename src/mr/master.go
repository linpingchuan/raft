package mr

import (
	"log"
	"net"
	"net/http"
	"net/rpc"
	"os"
	"sync"
)

const (
	MAP_STATUS = iota
	REDUCE_STATUS
)

type Master struct {
	// Your definitions here.
	sync.Mutex

	files      []string
	taskStatus map[string]int
}

// Your code here -- RPC handlers for the worker to call.

//
// an example RPC handler.
//
// the RPC argument and reply types are defined in rpc.go.
//
func (m *Master) Example(args *ExampleArgs, reply *ExampleReply) error {
	reply.Y = args.X + 1
	return nil
}

var waitGroup sync.WaitGroup

//
// 分发master的文件名到worker中
func (m *Master) SendTask(args *TaskArgs, reply *TaskReply) error {
	for idx, fileName := range m.files {
		waitGroup.Add(1)
		if _, ok := m.taskStatus[fileName]; ok {
			log.Println("task-status-filename: ", fileName)
			waitGroup.Done()
			continue
		}
		log.Println("taskStatus: ",m.taskStatus)
		reply.Filename = fileName
		reply.TaskIndex = idx
		m.taskStatus[fileName] = MAP_STATUS
		waitGroup.Done()
		break
	}
	waitGroup.Wait()
	log.Println("filename: ", reply.Filename, ",index: ", reply.TaskIndex)
	return nil
}

//
// start a thread that listens for RPCs from worker.go
//
func (m *Master) server() {
	rpc.Register(m)
	rpc.HandleHTTP()
	//l, e := net.Listen("tcp", ":1234")
	sockname := masterSock()
	log.Println("sock name: ", sockname)
	os.Remove(sockname)
	l, e := net.Listen("unix", sockname)
	if e != nil {
		log.Fatal("listen error:", e)
	}
	go http.Serve(l, nil)
}

//
// main/mrmaster.go calls Done() periodically to find out
// if the entire job has finished.
//
func (m *Master) Done() bool {
	ret := false

	// Your code here.

	return ret
}

//
// create a Master.
// main/mrmaster.go calls this function.
// nReduce is the number of reduce tasks to use.
//
func MakeMaster(files []string, nReduce int) *Master {
	m := Master{}

	// Your code here.
	// 将提交的任务files存在master结构中
	m.taskStatus = make(map[string]int)
	m.files = files

	m.server()
	return &m
}
