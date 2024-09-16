use crate::process;

use process::{enum_proc, Process, ProcessItem};

pub fn hack(address: u32, len: usize) -> Vec<u8> {
    let pids = enum_proc()
        .unwrap()
        .into_iter()
        .flat_map(Process::open)
        .flat_map(|proc| match proc.name() {
            Ok(name) => Ok(ProcessItem {
                pid: proc.pid(),
                name,
            }),
            Err(err) => Err(err),
        })
        .collect::<Vec<_>>();

    let pids = pids
        .into_iter()
        .filter(|p| p.name.to_lowercase().contains("war3"))
        .collect::<Vec<_>>();

    println!("match:{}", pids.len());
    let mut res = vec![];
    for ProcessItem { pid, name } in pids {
        println!("pid:{pid}, name:{name}");
        let process = Process::open(pid).unwrap();
        // let regions: Vec<MEMORY_BASIC_INFORMATION> = process.memory_regions();
        // println!("Scanning {} memory regions", regions.len());

        // let target = address.to_ne_bytes();
        // let mut locations = Vec::new();
        let mem = match process.read_memory(address as usize, 4 * len) {
            Ok(mem) => mem,
            Err(_) => panic!("read mem failure"),
        };

        for ele in mem {
            res.push(ele);
        }

        // for region in regions {
        //     match process.read_memory(region.BaseAddress as _, region.RegionSize) {
        //         Ok(mem) => mem
        //             .windows(target.len())
        //             .enumerate()
        //             .for_each(|(offset, window)| {
        //                 if window == target {
        //                     locations.push(region.BaseAddress as usize + offset);
        //                 }
        //             }),
        //         Err(e) => continue,
        //     }
        // }
    }

    res


    // for ele in &pids {
    //     println!("{} {}", ele.pid, ele.name)
    // }

    // println!("Pid:");
    // let mut input = String::new();
    // stdin().read_line(&mut input).unwrap();
    // let pid = input.trim().parse::<u32>().unwrap();
    // let process = Process::open(pid).unwrap();

    // let regions:Vec<MEMORY_BASIC_INFORMATION> = process.memory_regions();
    // println!("Scanning {} memory regions", regions.len());

    // println!("Which exact value to scan for?");

    // let mut input = String::new();
    // stdin().read_line(&mut input).unwrap();
    // let target: u32 = input.trim().parse::<u32>().unwrap();
    // let target = target.to_ne_bytes();

    // let mut locations = Vec::new();
    // for region in regions {
    //     match process.read_memory(region.BaseAddress as _, region.RegionSize){
    //         Ok(mem)=>{
    //             mem.windows(target.len()).enumerate().for_each(|(offset, window)| {
    //                 if window == target {
    //                     locations.push(region.BaseAddress as usize + offset);
    //                 }
    //             })
    //         },
    //         Err(e)=>continue
    //     }
    // }
    // println!("The number of value is {:?}",locations.len());
    // while locations.len() != 1 {
    //     println!("Next Scan value:");
    //     let mut input = String::new();
    //     stdin().read_line(&mut input).unwrap();
    //     let target: u32 = input.trim().parse::<u32>().unwrap();
    //     let target = target.to_ne_bytes();

    //     locations.retain(|addr| match process.read_memory(*addr, target.len()) {
    //         Ok(memory) => memory == target,
    //         Err(_) => false,
    //     });
    // }

    // println!("Scan Finished! this address is :{:?},Type the new value:",locations[0]);

    // let mut input = String::new();
    // stdin().read_line(&mut input).unwrap();
    // let new_value: i32 = input.trim().parse::<i32>().unwrap();
    // let new_value = new_value.to_ne_bytes();
    // process.write_memory(locations[0], &new_value);

    // println!("The new value is {:?}",new_value);
}
