extern crate mpi;

use mpi::traits::*;
use mpi::topology::Rank;
use mpi::collective::SystemOperation;

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();
    let size = world.size();
    let root_rank = 0;

    if rank == root_rank {
        let mut sum: Rank = 0;
        world.process_at_rank(root_rank)
            .immediate_reduce_into_root(&rank, &mut sum, &SystemOperation::sum())
            .wait();
        assert_eq!(sum, size * (size - 1) / 2);
    } else {
        world.process_at_rank(root_rank)
            .immediate_reduce_into(&rank, &SystemOperation::sum())
            .wait();
    }

    let mut max: Rank = -1;

    world.immediate_all_reduce_into(&rank, &mut max, &SystemOperation::max()).wait();
    assert_eq!(max, size - 1);

    let a = (0..size).collect::<Vec<_>>();
    let mut b: Rank = 0;

    world.immediate_reduce_scatter_block_into(&a[..], &mut b, &SystemOperation::product()).wait();
    assert_eq!(b, rank.pow(size as u32));
}
