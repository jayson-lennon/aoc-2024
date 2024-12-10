use std::ops::Index;

use derive_more::derive::From;

use crate::AocSolver;

pub struct Day09Solver;

impl AocSolver for Day09Solver {
    type Output = u64;

    fn part_1(input: &str) -> Self::Output {
        let mut storage = BlockStorage::from(input);
        storage.compact();

        checksum(&storage)
    }

    fn part_2(input: &str) -> Self::Output {
        let mut storage = BlockStorage::from(input);
        storage.defrag();

        checksum(&storage)
    }
}

fn checksum(storage: &BlockStorage) -> u64 {
    let mut sum: u64 = 0;
    for (i, block) in storage.iter().enumerate() {
        match block {
            Block::File(id) => {
                sum += i as u64 * id.0 as u64;
            }
            Block::Free => (),
        }
    }
    sum
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileId(pub usize);

impl From<usize> for FileId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Block {
    File(FileId),
    Free,
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Block::File(file_id) => write!(f, "{}", file_id.0),
            Block::Free => write!(f, "."),
        }
    }
}

#[derive(Debug)]
struct BlockStorage {
    inner: Vec<Block>,
}

impl BlockStorage {
    fn iter(&self) -> impl Iterator<Item = &Block> {
        self.into_iter()
    }

    fn compact(&mut self) {
        // start the "free space index" at the first available free space
        let mut free_space_idx = {
            let mut i = 0;
            for block in &self.inner {
                if matches!(block, Block::Free) {
                    break;
                }
                i += 1;
            }
            i
        };

        // the "block index" starts from the last block and moves backwards
        let mut block_idx = self.inner.len() - 1;

        'swap: loop {
            if free_space_idx >= block_idx {
                break;
            }
            self.inner.swap(free_space_idx, block_idx);

            // increment the free space index until we find the next free space
            while matches!(self.inner[free_space_idx], Block::File(_)) {
                // reaching the block index means we are done (no more blocks to move)
                if free_space_idx == self.inner.len() - 1 {
                    break 'swap;
                }
                free_space_idx += 1;
            }

            // decrement the block index until we find the next block
            while matches!(self.inner[block_idx], Block::Free) {
                // reaching the free space index means we are done (no more free space available)
                if block_idx == 0 {
                    break 'swap;
                }
                block_idx -= 1;
            }
        }
    }

    fn iter_free_space(&self) -> FreeSpaceIterator {
        FreeSpaceIterator {
            current_index: 0,
            storage: self,
        }
    }

    fn iter_files(&self) -> FileIterator {
        FileIterator {
            current_index: self.inner.len() - 1,
            storage: self,
        }
    }

    fn defrag(&mut self) {
        let files = self.iter_files().collect::<Vec<_>>();
        for file in files {
            let mut free_space = self.iter_free_space();
            // eprintln!(
            //     "check file: {} size {}",
            //     self.inner[file.index()],
            //     file.size()
            // );
            if let Some(space) = free_space.find(|space| file.size() <= space.size()) {
                if space.index() >= file.index() {
                    continue;
                }
                // eprintln!("BEFORE SWAP: {self}");
                // eprintln!(
                //     "SWAPPPPP Space avail: {}, required: {}. swapping {:?} -> {:?}",
                //     space.size(),
                //     file.size(),
                //     space,
                //     file
                // );
                self.swap_chunk(space, file);
                // eprintln!("AFTER SWAP: {self}");
            }
        }
    }

    fn swap_chunk<A, B>(&mut self, space: A, file: B)
    where
        A: BlockInfo,
        B: BlockInfo,
    {
        for s in 0..file.size() {
            self.inner.swap(space.index() + s, file.index() + s);
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

trait BlockInfo {
    fn index(&self) -> usize;
    fn size(&self) -> usize;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct FreeSpaceEntry {
    at: usize,
    size: usize,
}

impl BlockInfo for FreeSpaceEntry {
    fn index(&self) -> usize {
        self.at
    }

    fn size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
struct FreeSpaceIterator<'a> {
    current_index: usize,
    storage: &'a BlockStorage,
}

impl<'a> Iterator for FreeSpaceIterator<'a> {
    type Item = FreeSpaceEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = self.current_index;
        if i == self.storage.len() - 1 {
            return None;
        }
        // skip file blocks
        while self.storage[i] != Block::Free {
            i += 1;
            self.current_index += 1;
            if i == self.storage.len() - 1 {
                break;
            }
        }
        let free_space_start = i;

        let mut size = 0;
        // track free blocks
        while self.storage[i] == Block::Free {
            size += 1;
            i += 1;
            self.current_index += 1;
            if i == self.storage.len() - 1 {
                break;
            }
        }

        (size > 0).then_some(FreeSpaceEntry {
            at: free_space_start,
            size,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct FileEntry {
    at: usize,
    size: usize,
}

impl BlockInfo for FileEntry {
    fn index(&self) -> usize {
        self.at
    }

    fn size(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
struct FileIterator<'a> {
    current_index: usize,
    storage: &'a BlockStorage,
}

impl<'a> Iterator for FileIterator<'a> {
    type Item = FileEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let mut i = self.current_index;
        // skip free space if we left off on free space on a previous iteration
        while self.storage[i] == Block::Free {
            i -= 1;
            self.current_index -= 1;
        }

        let current = self.storage[i];
        let mut size = 0;
        while self.storage[i] == current {
            if i == 0 {
                break;
            }
            self.current_index -= 1;
            size += 1;
            i -= 1;
        }

        (size > 0).then_some(FileEntry { at: i + 1, size })
    }
}

impl std::fmt::Display for BlockStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for block in &self.inner {
            write!(f, "{}", block)?;
        }
        Ok(())
    }
}

impl IntoIterator for BlockStorage {
    type Item = Block;
    type IntoIter = std::vec::IntoIter<Block>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a BlockStorage {
    type Item = &'a Block;
    type IntoIter = std::slice::Iter<'a, Block>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl From<&str> for BlockStorage {
    fn from(raw: &str) -> Self {
        BlockStorage {
            inner: {
                // ignore newline at EOF, if it exists
                let raw = {
                    if raw.ends_with('\n') {
                        &raw[0..raw.len() - 1]
                    } else {
                        raw
                    }
                };
                let mut block_storage = Vec::default();
                // use exact size here and then handle the final one (single number) later
                for (id, chunk) in raw.as_bytes().chunks_exact(2).enumerate() {
                    let (size, free_space) = (char_to_num(chunk[0]), char_to_num(chunk[1]));
                    for _ in 0..size {
                        block_storage.push(Block::File(id.into()));
                    }
                    for _ in 0..free_space {
                        block_storage.push(Block::Free);
                    }
                }
                // handle final element
                {
                    let bytes = raw.as_bytes();
                    for _ in 0..char_to_num(bytes[bytes.len() - 1]) {
                        block_storage.push(Block::File((bytes.len() / 2).into()));
                    }
                }
                block_storage
            },
        }
    }
}

impl Index<usize> for BlockStorage {
    type Output = Block;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

fn char_to_num(b: u8) -> u8 {
    match b {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::owo_colors::OwoColorize;

    use super::*;

    const SAMPLE: &str = r#"2333133121414131402
"#;

    #[test]
    fn parses_into_block_storage() {
        let storage = BlockStorage::from(SAMPLE);

        assert_eq!(
            storage.to_string(),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn compacts_block_storage() {
        let mut storage = BlockStorage::from(SAMPLE);
        storage.compact();

        assert_eq!(storage[0], Block::File(FileId(0)));
        assert_eq!(storage[1], Block::File(FileId(0)));
        assert_eq!(storage[2], Block::File(FileId(9)));
        assert_eq!(storage[3], Block::File(FileId(9)));
        assert_eq!(storage[4], Block::File(FileId(8)));
    }

    #[test]
    fn solves_part_1() {
        let answer = Day09Solver::part_1(SAMPLE);

        assert_eq!(answer, 1928);
    }

    #[test]
    fn iter_free_space() {
        let storage = BlockStorage::from(SAMPLE);

        let mut free_space = storage.iter_free_space();

        assert_eq!(free_space.next(), Some(FreeSpaceEntry { at: 2, size: 3 }));
        assert_eq!(free_space.next(), Some(FreeSpaceEntry { at: 8, size: 3 }));
        assert_eq!(free_space.next(), Some(FreeSpaceEntry { at: 12, size: 3 }));
        assert_eq!(free_space.next(), Some(FreeSpaceEntry { at: 18, size: 1 }));
    }

    #[test]
    fn iter_files() {
        let storage = BlockStorage::from(SAMPLE);

        let mut files = storage.iter_files();

        assert_eq!(files.next(), Some(FileEntry { at: 40, size: 2 }));
        assert_eq!(files.next(), Some(FileEntry { at: 36, size: 4 }));
        assert_eq!(files.next(), Some(FileEntry { at: 32, size: 3 }));
        assert_eq!(files.nth(4), Some(FileEntry { at: 11, size: 1 }));
    }

    #[test]
    fn defrags() {
        let mut storage = BlockStorage::from(SAMPLE);

        storage.defrag();

        assert_eq!(
            storage.to_string(),
            "00992111777.44.333....5555.6666.....8888.."
        );
    }

    #[test]
    fn solves_part_2() {
        let answer = Day09Solver::part_2(SAMPLE);

        assert_eq!(answer, 2858);
    }
}
