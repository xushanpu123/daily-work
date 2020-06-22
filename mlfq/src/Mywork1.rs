extern crate rand;
use rand::Rng;
use std::collections::Hashmap;
struct info{
    seed: i32,
    numQueues: i32,
    quantum:i32,
    allotment:i32,
    quantumList:String,
    allotmentList:String,
    numJobs:i32,
    maxlen:i32,
    maxio:i32,
    boost:i32,
    iotime:i32,
    stay:bool,
    iobump:bool,
    jlist:String,
    solve:bool
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
struct done
{
    num:i32,
    type:&str
}
fn main()
{
let mut options = Site{
    seed: 0,
    numQueues: 3,
    quantum:10,
    allotment:1,
    quantumList:String::from(''),
    allotmentList:String::from(''),
    numJobs:3,
    maxlen:100,
    maxio:100,
    boost:2,
    iotime:5,
    stay:false,
    iobump:false,
    jlist:String::from(''),
    solve:false;
}
let mut quantum = Hashmap::new();
if options.quantumList!=''
{
   let quantumLengths:Vec<&str> = options.quantumList.split(",").collect();
   numQueues =  quantumLengths.len();
   let mut qc = numQueues - 1;
   let mut i = 0;
   while i<numQueues{
       quantum.get_mut(&qc).unwrap() = String::from(quantumLengths[i]).parse::<i32>().unwrap();
       i+=1;
       qc-=1;
   }
}
else
{
    let mut i = 0;
    while i<numQueues
    {
        quantum.get_mut(&i).unwrap() = options.quantum;
        i+=1;
    }
}
let mut allotment = Hashmap::new();
if options.allotmentList!=''
{
   let allotmentLengths:Vec<&str> = options.allotmentList.split(",").collect();
   if numQueues != allotmentLengths.len()
   {
       println!("number of allotments specified must match number of quantums");
       exit(1);
   }
   let mut qc = numQueues - 1;
   let mut i = 0;
   while i<numQueues{
       allotment.get_mut(&qc).unwrap() = String::from(allotmentLengths[i]).parse::<i32>().unwrap();
       i+=1;
       qc-=1;
   }
}
else
{
    let mut i = 0;
    while i<numQueues
    {
        allotment.get_mut(&i).unwrap() = options.allotment;
        i+=1;
    }
}
let hiQueue = numQueues - 1;
let mut iotime = options.iotime;
let mut ioDone = Hashmap::new();
let mut job = Hashmap::new();
let mut jobCnt = 0;
let mut allJobs::Vec<&str>=options.jlist.split(':').collect();
let mut job:[JOB;100] =[JOB({0,0,0,0,0,0,false,0})];
for j in allJobs.iter(){
    let mut jobInfo:Vec<&str> = j.split(',').collect();
    if jobInfo.len()!=3
    {
        println!('Badly formatted job string. Should be x1,y1,z1:x2,y2,z2:...')

        ​      println!('where x is the startTime, y is the runTime, and z is the I/O frequency.')
        
        ​      exit(1)
    }
    asserteq!(jobInfo.len(),3);
    let startTime = String::from(jobInfo[0]).parse::<i32>().unwrap();
    let runTime = String::from(jobInfo[1]).parse::<i32>().unwrap();
    let ioFreq = String::from(jobInfo[2]).parse::<i32>().unwrap();
    job[jobCnt] =JOB{
        currPri:hiQueue,
        ticksLeft:quantum[hiQueue],
        allotLeft:allotment[hiQueue],
        startTime:startTime,
        runTime:runTime,
        timeLeft:runTime,
        ioFreq:ioFreq,
        doingIO:false,
        firstRun:-1
    }
    if ioDone.get(startTime)==None
    {
        let elem = done{
            num:startTime,
            type:"JOB BEGINS"
        }
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
println!("OPTIONS ioTime             {:?}",options.ioTime);
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

if options.solve == False{
    println!("Compute the execution trace for the given workloads.");
    println!("If you would like, also compute the response and turnaround");
    println!("times for each of the jobs.");
    println!("");
    println!("Use the -c flag to get the exact results when you are finished.\n");
    exit(0)
}
let mut queue = Hashmap::new();
let mut i=0;
while i<numQueues{
    let date::Vec<i32>==Vec::new();
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
            for j in queue.get_mut(&q).unwrap().iter(){
                if job[j].doingIO==false
                   {
                      queue.get_mut(&hiQueue).unwrap().push(&j); 
                   }
            }
            queue.get_mut(&q).unwrap().clear();
        }
        for j in 0..numJobs-1{
            if job.get_mut(&j).unwrap().timeLeft>0{
                job.get_mut(&j).unwrap().currPri=&hiQueue;
                job.get_mut(&j).unwrap().allotment=allotment.get_mut(&hiQueue).unwrap();
            }
        }
        if ioDone.get_mut(&currTime)!=None{
            for elem in ioDone.get_mut(&currTime).unwrap(). iter(){
                let j=&elem.num;
                let q=job.get_mut(&j).unwrap().currPri;
                job.get_mut(&j).unwrap().doingIO=false;
                println!("[ time %d ] %s by JOB %d",currTime,&elem.type,j);
                if options.iobump == False or type == 'JOB BEGINS'
                queue.get_mut(&q).unwrap().push(&j)
                else
                queue.get_mut(&q).unwrap().insert(0, &j);

            }
        }
        let currQueue = FindQueue()
        if currQueue == -1{
        println!("[ time {:?} ] IDLE",currTime);
        currTime += 1
        continue
        }
    // there was at least one runnable job, and hence ...
    currJob = queue.get_mut(&currQueue).unwrap().get_mut(&0).unwrap();
    if job.get_mut(&currJob).unwrap().currPri!= currQueue{
        println!("currPri[{:?}] does not match currQueue[{:?}]"% (job.get_mut(&currJob).unwrap().currPri, currQueue))
        exit(1);
    }
    job.get_mut(&currJob).unwrap().timeLeft  -= 1
    job.get_mut(&currJob).unwrap().ticksLeft -= 1

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
    currTime += 1

    // CHECK FOR JOB ENDING
    if timeLeft == 0{
        println!("[ time {:?} ] FINISHED JOB {:?}",currTime, currJob)
        finishedJobs += 1
        job.get_mut(&currJob).unwrap().endtime = currTime
        // print 'BEFORE POP', queue
        let Done = queue.get_mut(&currQueue).unwrap().get_mut(&0);
        queue.get_mut(&currQueue).unwrap().erase(0);
        // print 'AFTER POP', queue
        assert(Done == currJob)
        continue
    } 
    // CHECK FOR IO
    let issuedIO = false
    if ioFreq > 0 and (((runTime - timeLeft) % ioFreq) == 0){
        // time for an IO!
        println!("[ time %d ] IO_START by JOB %d",currTime, currJob);
        issuedIO = True
        desched = queue.get_mut(&currQueue).unwrap().get_mut(&0);
        queue.get_mut(&currQueue).unwrap().erase(0);
        assert(desched == currJob)
        job.get_mut(&currJob).unwrap().doingIO = true;
        // this does the bad rule -- reset your tick counter if you stay at the same level
        if options.stay == true{
            job.get_mut(&currJob).unwrap().ticksLeft = quantum[currQueue];
            job.get_mut(&currJob).unwrap().allotLeft = allotment[currQueue]
        }
        // add to IO Queue: but which queue?
        let futureTime = currTime + ioTime
        if ioDone.get(&futureTime)==None{
            let mut vec:Vec<done> =Vec::new();
            ioDone.insert(futureTime,vec);
        }
        print 'IO DONE'
        ioDone.get_mut(futureTime).push(done{num:&currJob,type:"IO_DONE"};
    }   
    // CHECK FOR QUANTUM ENDING AT THIS LEVEL (BUT REMEMBER, THERE STILL MAY BE ALLOTMENT LEFT)
    if ticksLeft == 0{
        if issuedIO == false{
            // IO HAS NOT BEEN ISSUED (therefor pop from queue)'
            desched = queue.get_mut(&currQueue).unwrap().get_mut(&0);
            queue.get_mut(&currQueue).unwrap().erase(0);
        }
        assert(desched == currJob)

        job.get_mut(&currJob).unwrap().allotLeft = job.get_mut(&currJob).unwrap().allotLeft - 1;

        if job.get_mut(&currJob).unwrap().allotLeft == 0{
            // this job is DONE at this level, so move on
            if currQueue > 0{
                // in this case, have to change the priority of the job
                job.get_mut(&currJob).unwrap().currPri   = currQueue - 1;
                job.get_mut(&currJob).unwrap().ticksLeft = quantum.get_mut(&currQueue-1).unwrap();
                job.get_mut(&currJob).unwrap().allotLeft = allotment.get_mut(&currQueue-1).unwrap();
                if issuedIO == false
                    queue.get_mut(&currQueue-1).unwrap().push(&currJob);
            }
            else{
                job.get_mut(&currJob).unwrap().ticksLeft = quantum.get_mut(&currQueue).unwrap();
                job.get_mut(&currJob).unwrap().allotLeft = allotment.get_mut(&currQueue).unwrap();
                }
                if issuedIO == false
                    queue.get_mut(&currQueue).unwrap().push(currJob);
        }
    }
        else
            // this job has more time at this level, so just push it to end
            job.get_mut(&currJob).unwrap().ticksLeft = quantum.get_mut(&currQueue).unwrap();
                if issuedIO == false
                   queue.get_mut(&currQueue).unwrap().push(currJob);
    }
}
//print out statistics
println!("");
println!("Final statistics:");
let mut responseSum   = 0
let mut turnaroundSum = 0
for i in 0..numJobs-1{
    let response   = job.get_mut(&i).unwrap().firstRun - job.get_mut(&i).unwrap().startTime;
    let turnaround = job.get_mut(&i).unwrap().endtime - job.get_mut(&i).unwrap().startTime;
    println!("Job {:?}: startTime {:?} - response {:?} - turnaround {:?}",i,job.get_mut(&i).unwrap().startTime,response, turnaround);
    responseSum   += response;
    turnaroundSum += turnaround;
}
println!("\n  Avg {:?}: startTime n/a - response {:?} - turnaround {:?}" ,i, responseSum/numJobs,turnaroundSum/numJobs);

println!("\n");
}
