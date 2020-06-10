from ctypes import *
import os,time,sys,datetime,socket,struct

def check_sandbox_in_process():
    EvidenceOfSandbox = []
    sandbox_processes = "vmsrvc", "tcpview", "wireshark", "visual basic", "fiddler", "vmware", "vbox", "process explorer", "autoit", "vboxtray", "vmtools", "vmrawdsk", "vmusbmouse", "vmvss", "vmscsi", "vmxnet", "vmx_svga", "vmmemctl", "df5serv", "vboxservice", "vmhgfs", "vmtoolsd"
    runningProcess = []
    for item in os.popen("tasklist").read().splitlines()[4:]:
        runningProcess.append(item.split())
    # for process in runningProcess:
    #     for sandbox_process in sandbox_processes:
    #         if sandbox_process in process:
    #             if process not in EvidenceOfSandbox:
    #                 EvidenceOfSandbox.append(process)
    #                 break
    for each in runningProcess:
        print(each)

check_sandbox_in_process()