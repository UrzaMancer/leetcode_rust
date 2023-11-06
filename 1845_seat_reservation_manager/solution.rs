/ 368 ms 25.00 %
// 28.24 MB 75.00%

struct SeatManager {
    seats: Vec<Option<i32>>,
    // note the naive option is probably bool, for whether the seat is reserved,
    // but using an Option that can store the 1-indexed seat number works a
    // little better with program logic when using the find() iterator adapter,
    // then we store Some(seat_number) for all available seats,
    // and None for "seat not available"
    smallest_open_seat: i32,
    // I think it would be better to write this as an Option,
    // but I skipped it for this example because the problem
    // constraints guarantee reserve() will always be valid

    // I also briefly considered switching from Vec for seats
    // to using something like a min-heap, but these seemed
    // quickest to implement.
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {

    fn new(n: i32) -> Self {
        Self {
            seats: (1..=n).map(|seat| Some(seat)).collect(),
            smallest_open_seat: 1,
        }
    }
    
    fn reserve(&mut self) -> i32 {
        let reservation: usize = self.smallest_open_seat as usize - 1;
        self.seats[reservation] = None;
        let now_reserved = self.smallest_open_seat;
        
        self.smallest_open_seat = {
            if let Some(open_seat) = 
                self.seats[reservation..].iter()
                    //.inspect(|thing| {print!("{:?} ", thing);})
                    .find_map(|&seat| seat)
                { open_seat }
            else
                { self.seats.len() as i32 + 1 }
            // I would prefer not to use this magic number, and
            // instead use an option, but this was slightly quicker
        };

        //println!("");
        //println!("  r -> {:?}", self.seats);
        now_reserved
    }
    
    fn unreserve(&mut self, seat_number: i32) {
        if seat_number < self.smallest_open_seat {
            self.smallest_open_seat = seat_number;
        }
        self.seats[seat_number as usize - 1] = Some(seat_number);
        //println!("  u{} -> {:?}", seat_number, self.seats)
    }
}

/**
 * Your SeatManager object will be instantiated and called as such:
 * let obj = SeatManager::new(n);
 * let ret_1: i32 = obj.reserve();
 * obj.unreserve(seatNumber);
 */
