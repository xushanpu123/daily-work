
use std::collections::HashMap;
struct Options {
    seed: i32,
    numQueues: i32,
    quantum: i32,
    allotment: i32,
    quantumList: String,
    allotmentList: String,
    numJobs: i32,
    maxlen: i32,
    maxio: i32,
    boost: i32,
    iotime: i32,
    stay: bool,
    iobump: bool,
    jlist: String,
    solve: bool
}
struct done
{
    num:i32,
    Type:String,
}
struct JOB{
    currPri:i32,
    ticksLeft:i32,
    allotLeft:i32,
    startTime:i32,
    runTime:i32,
    timeLeft:i32,
    ioFreq:i32,
    doingIO:bool,
    firstRun:i32
}

fn FindQueue(hiQueue: i32, queue: HashMap<i32, Vec<i32>>) -> i32 {

    let mut q =  hiQueue;

    while q > 0 {

        if queue.get(&q).unwrap().len() > 0 {
            return q;
        }
        q -= 1;
    }

    if queue.get(&0).unwrap().len() > 0 {
        return 0;
    }

    return -1;
}

fn main() {
    
    let options:Options = Options{
        seed: 0,
        numQueues: 3,
        quantum: 10,
        allotment: 1,
        quantumList: String::from(""),
        allotmentList: String::from(""),
        numJobs: 3,
        maxlen: 100,
        maxio: 10,
        boost: 0,
        iotime: 5,
        stay: false,
        iobump: false,
        jlist: String::from(""),
        solve: false
    };

    let mut numQueues = options.numQueues;

    let mut queue = HashMap::new();
    
    
    for i in 0..numQueues {


        let v: Vec<i32> = Vec::new();

        queue.insert(i, v);

    }
    
    let mut quantum = HashMap::new();
if options.quantumList!=""
{
   let quantumLengths:Vec<&str> = options.quantumList.split(",").collect();
   numQueues =  quantumLengths.len() as i32;
   let mut qc = numQueues - 1;
   let mut i= 0;
   while i<numQueues as usize{
       let mut v =String::from(quantumLengths[i]).parse::<i32>().unwrap();
       quantum.insert(qc, v);
       i+=1;
       qc-=1;
   }
}
else
{
    let mut i = 0;
    while i<numQueues
    {
        quantum.insert(i,options.quantum);
        i+=1;
    }
}
let mut allotment = HashMap::new();
if options.allotmentList!=""
{
   let allotmentLengths:Vec<&str> = options.allotmentList.split(",").collect();
   if numQueues != allotmentLengths.len() as i32
   {
       println!("number of allotments specified must match number of quantums");
   }
   let mut qc = numQueues - 1;
   let mut i = 0;
   while i<numQueues{
       let mut v =  String::from(allotmentLengths[i as usize]).parse::<i32>().unwrap();
       allotment.insert(qc, v);
       i+=1;
       qc-=1;
   }
}
else
{
    let mut i = 0;
    while i<numQueues
    {
        allotment.insert(i,options.allotment);
        i+=1;
    }
}
let hiQueue = numQueues - 1;
let mut iotime = options.iotime;
let mut ioDone = HashMap::new();
let mut job = HashMap::new();
let mut jobCnt = 0;
let mut allJobs:Vec<&str>=options.jlist.split(':').collect();
for j in allJobs.iter(){
    let mut jobInfo:Vec<&str> = j.split(',').collect();
    if jobInfo.len()!=3
    {
        println!("Badly formatted job string. Should be x1,y1,z1:x2,y2,z2:...");

        ​ println!("where x is the startTime, y is the runTime, and z is the I/O frequency.");
        
        ​    
    }
    assert_eq!(jobInfo.len(),3);
    let startTime = String::from(jobInfo[0]).parse::<i32>().unwrap();
    let runTime = String::from(jobInfo[1]).parse::<i32>().unwrap();
    let ioFreq = String::from(jobInfo[2]).parse::<i32>().unwrap();
    job.insert(jobCnt, JOB{
        currPri:hiQueue,
        ticksLeft:*quantum.get(&hiQueue).unwrap(),
        allotLeft:*allotment.get(&hiQueue).unwrap(),
        startTime:startTime,
        runTime:runTime,
        timeLeft:runTime,
        ioFreq:ioFreq,
        doingIO:false,
        firstRun:-1
    });
    if ioDone.get(&startTime)==None
    {
        let elem = done{
            num:startTime,
            Type:String::from("JOB BEGINS"),
        };
        ioDone.insert(startTime,elem);
    }
    jobCnt+=1;
}
let numJobs = job.len();

println!( "Here is the list of inputs:");
println!("OPTIONS jobs           {:?}",numJobs);
println!("OPTIONS queue          {:?}",numQueues); 
let mut i = numQueues - 1;
while i>-1{
    println!("OPTIONS allotments for queue {:?} is {:?}",i,allotment[i]);
    println!("OPTIONS quantum length for queue {:?} is {:?}",i,quantum[i]);
}
println!("OPTIONS boost              {:?}",options.boost);
println!("OPTIONS ioTime             {:?}",options.iotime);
println!("OPTIONS stayAfterIO        {:?}",options.stay);
println!("OPTIONS iobump             {:?}",options.iobump);
println!("");
println!("For each job, three defining characteristics are given:");
println!("  startTime : at what time does the job enter the system");
println!("runTime   : the total CPU time needed by the job to finish");
println!("  ioFreq    : every ioFreq time units, the job issues an I/O");
println!("              (the I/O takes ioTime units to complete)\n");
println!("Job List");
let mut i=0;
while i<numJobs{
    println!("  Job {:?}: startTime {:?} - runTime {:?} - ioFreq {:?}",i,job[i][startTime],job[i][runTime],jov[i][ioFreq];
}

println!("");

if options.solve == false{
    println!("Compute the execution trace for the given workloads.");
    println!("If you would like, also compute the response and turnaround");
    println!("times for each of the jobs.");
    println!("");
    println!("Use the -c flag to get the exact results when you are finished.\n");
}
let mut queue = HashMap::new();
let mut i=0;
while i<numQueues{
    let date:Vec<i32>=Vec::new();
    queue.insert(i,date);
    i+=1;
}
let currTime=0;
let totalJobs=job.len();
let finishJobs=0;
println!("\nExcution trace:\n");
while finishJobs<totalJobs{
    if currTime%options.boost==0{
        println!("[time {:?}]BOOST (every {:?})",currTime,options.boost);
        for q in 0..numQueues-2{
            for j in queue.get(&q).unwrap().iter(){
                if job[j].doingIO==false
                   {
                      queue.get(&hiQueue).unwrap().push(&j); 
                   }
            }
            queue.get_mut(&q).unwrap().clear();
        }
        for j in 0..numJobs-1 {
            if job.get_mut(&(j as i32)).unwrap().timeLeft>0{
                job.get_mut(&(j as i32)).unwrap().currPri=hiQueue;
                job.get_mut(&(j as i32)).unwrap().allotLeft=*allotment.get(&hiQueue).unwrap();
            }
        }
        if ioDone.get_mut(&currTime)!=None{
            for elem in ioDone.get(&currTime).unwrap().iter(){
                let j=elem.num;
                let q=job.get_mut(&j).unwrap().currPri;
                job.get_mut(&j).unwrap().doingIO=false ;
                println!("[ time {:?} ] {:?} by JOB {:?}",currTime,elem.Type,j);
                if options.iobump == false || elem.Type == "JOB BEGINS"
                {
                queue.get_mut(&q).unwrap().push(j);
                }
                else
                {
                queue.get_mut(&q).unwrap().insert(0, j);
                }

            }
        }
        let currQueue = FindQueue(hiQueue,queue);
        if currQueue == -1{
        println!("[ time {:?} ] IDLE",currTime);
        currTime += 1;
        continue;
        }
    // there was at least one runnable job, and hence ...
    let currJob = queue.get_mut(&currQueue).unwrap().get(&0).unwrap();
    if job.get_mut(&currJob).unwrap().currPri!= currQueue{
        println!("currPri[{:?}] does not match currQueue[{:?}]",job.get_mut(&currJob).unwrap().currPri, currQueue);
    }
    job.get_mut(&currJob).unwrap().timeLeft  -= 1;
    job.get_mut(&currJob).unwrap().ticksLeft -= 1;

    if job.get_mut(&currJob).unwrap().firstRun == -1{
        job.get_mut(&currJob).unwrap().firstRun = currTime;
    }
    let runTime   = job.get_mut(&currJob).unwrap().runTime;
    let ioFreq    = job.get_mut(&currJob).unwrap().ioFreq;
    let ticksLeft = job.get_mut(&currJob).unwrap().ticksLeft;
    let allotLeft = job.get_mut(&currJob).unwrap().allotLeft;
    let timeLeft  = job.get_mut(&currJob).unwrap().timeLeft;

    println!("[ time {:?} ] Run JOB {:?} at PRIORITY {:?} [ TICKS {:?} ALLOT {:?} TIME {:?} (of {:?}) ]",currTime, currJob, currQueue, ticksLeft, allotLeft, timeLeft, runTime);

    if timeLeft < 0
    {
        println!("Error: should never have less than 0 time left to run");
        exit(1);
    }

    // UPDATE TIME
    currTime += 1;

    // CHECK FOR JOB ENDING
    if timeLeft == 0{
        println!("[ time {:?} ] FINISHED JOB {:?}",currTime, currJob);
        finishedJobs += 1;
        job.get_mut(&currJob).unwrap().endtime = currTime;
        // print 'BEFORE POP', queue
        let Done = queue.get_mut(&currQueue).unwrap().get_mut(&0);
        queue.get_mut(&currQueue).unwrap().erase(0);
        // print 'AFTER POP', queue
        assert(Done == currJob);
        continue
    } 
    // CHECK FOR IO
    let issuedIO = false;
    if ioFreq > 0 && (((runTime - timeLeft) % ioFreq) == 0){
        // time for an IO!
        println!("[ time %d ] IO_START by JOB %d",currTime, currJob);
        issuedIO = true;
        desched = queue.get_mut(&currQueue).unwrap().get_mut(&0);
        queue.get_mut(&currQueue).unwrap().erase(0);
        assert(desched == currJob);
        job.get_mut(&currJob).unwrap().doingIO = true;
        // this does the bad rule -- reset your tick counter if you stay at the same level
        if options.stay == true{
            job.get_mut(&currJob).unwrap().ticksLeft = quantum[currQueue];
            job.get_mut(&currJob).unwrap().allotLeft = allotment[currQueue]
        }
        // add to IO Queue: but which queue?
        let futureTime = currTime + ioTime;
        if ioDone.get(&futureTime)==None{
            let mut vec:Vec<done> =Vec::new();
            ioDone.insert(futureTime,vec);
        }
        println!("IO DONE");
        ioDone.get_mut(futureTime).push(done{num:&currJob,Type:"IO_DONE"});
    }   
    // CHECK FOR QUANTUM ENDING AT THIS LEVEL (BUT REMEMBER, THERE STILL MAY BE ALLOTMENT LEFT)
    if ticksLeft == 0{
        if issuedIO == false{
            // IO HAS NOT BEEN ISSUED (therefor pop from queue)'
            desched = queue.get_mut(&currQueue).unwrap().get_mut(&0);
            queue.get_mut(&currQueue).unwrap().erase(0);
        }
        assert(desched == currJob);

        job.get_mut(&currJob).unwrap().allotLeft = job.get_mut(&currJob).unwrap().allotLeft - 1;

        if job.get_mut(&currJob).unwrap().allotLeft == 0{
            // this job is DONE at this level, so move on
            if currQueue > 0{
                // in this case, have to change the priority of the job
                job.get_mut(&currJob).unwrap().currPri   = currQueue - 1;
                job.get_mut(&currJob).unwrap().ticksLeft = quantum.get_mut(&(currQueue-1)).unwrap();
                job.get_mut(&currJob).unwrap().allotLeft = allotment.get_mut(&(currQueue-1)).unwrap();
                if issuedIO == false
                {
                    queue.get_mut(&(currQueue-1)).unwrap().push(currJob);
                }
            }
            else{
                job.get_mut(&currJob).unwrap().ticksLeft = quantum.get_mut(&currQueue).unwrap();
                job.get_mut(&currJob).unwrap().allotLeft = allotment.get_mut(&currQueue).unwrap();
                }
                if issuedIO == false
                {
                    queue.get_mut(&currQueue).unwrap().push(currJob);
                }
        }
    }
        else
         {
            // this job has more time at this level, so just push it to end
            job.get_mut(&currJob).unwrap().ticksLeft = quantum.get_mut(&currQueue).unwrap();
                if issuedIO == false
                {
                   queue.get_mut(&currQueue).unwrap().push(currJob);
                }
         }
    }
}
//print out statistics
println!("");
println!("Final statistics:");
let mut responseSum   = 0;
let mut turnaroundSum = 0;
for i in 0..numJobs-1{
    let response   = job.get_mut(&(i as i32)).unwrap().firstRun - job.get_mut(&(i as i32)).unwrap().startTime;
    let turnaround = job.get_mut(&(i as i32)).unwrap().endtime - job.get_mut(&(i as i32)).unwrap().startTime;
    println!("Job {:?}: startTime {:?} - response {:?} - turnaround {:?}",i,job.get_mut(&(i as i32)).unwrap().startTime,response, turnaround);
    responseSum   += response;
    turnaroundSum += turnaround;
}
println!("\n  Avg {:?}: startTime n/a - response {:?} - turnaround {:?}" ,i, responseSum/(numJobs as i32),turnaroundSum/numJobs);

println!("\n");
}
