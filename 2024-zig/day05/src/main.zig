const std = @import("std");

const Book = struct {
    const Self = @This();

    pages: std.ArrayList(u32),

    pub fn init(allocator: std.mem.Allocator) !Self {
        return Self{
            .pages = try std.ArrayList(u32).initCapacity(allocator, 1024),
        };
    }

    pub fn deinit(self: Self) void {
        self.pages.deinit();
    }

    pub fn add(self: *Self, page: u32) !void {
        try self.pages.append(page);
    }

    pub fn items(self: *const Self) []u32 {
        return self.pages.items;
    }
};

pub fn main() !void {
    const file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var br = std.io.bufferedReader(file.reader());
    const in_stream = br.reader();

    var buf: [1024]u8 = undefined;

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const allocator = gpa.allocator();

    var rules = std.AutoHashMap(u32, std.ArrayList(u32)).init(allocator);
    defer {
        var it = rules.iterator();
        while (it.next()) |entry| {
            entry.value_ptr.*.deinit();
        }
        rules.deinit();
    }

    var books = try std.ArrayList(Book).initCapacity(allocator, 512);
    defer {
        for (books.items) |book| {
            book.deinit();
        }
        books.deinit();
    }

    while (true) {
        var fbs = std.io.fixedBufferStream(&buf);
        const result = in_stream.streamUntilDelimiter(fbs.writer(), '\n', fbs.buffer.len);

        if (result) |_| {
            const written = fbs.getWritten();
            if (written.len == 0)
                break;

            var it = std.mem.splitScalar(u8, written, '|');
            const pre_buf = it.next() orelse return error.MissingFirst;
            const pre: u32 = try std.fmt.parseInt(u32, pre_buf, 10);

            const post_buf = it.next() orelse return error.MissingFirst;
            const post: u32 = try std.fmt.parseInt(u32, post_buf, 10);

            const gop = try rules.getOrPut(pre);
            if (!gop.found_existing)
                gop.value_ptr.* = try std.ArrayList(u32).initCapacity(allocator, 10);
            try gop.value_ptr.*.append(post);
        } else |err| {
            if (err == error.EndOfStream) {
                break;
            } else {
                return err;
            }
        }
    }

    while (true) {
        var fbs = std.io.fixedBufferStream(&buf);
        const result = in_stream.streamUntilDelimiter(fbs.writer(), '\n', fbs.buffer.len);

        if (result) |_| {
            const written = fbs.getWritten();

            var book = try Book.init(allocator);

            var it = std.mem.splitScalar(u8, written, ',');
            while (it.next()) |entry| {
                const page: u32 = try std.fmt.parseInt(u32, entry, 10);
                try book.add(page);
            }

            try books.append(book);
        } else |err| {
            if (err == error.EndOfStream) {
                break;
            } else {
                return err;
            }
        }
    }

    // Day 05 Star 1.
    var count1: u32 = 0;
    book: for (books.items) |book| {
        var seen = std.AutoHashMap(u32, void).init(allocator);
        defer seen.deinit();

        for (book.items()) |page| {
            const post = rules.get(page);
            if (post) |post_pg| {
                for (post_pg.items) |pg| {
                    if (seen.contains(pg))
                        continue :book;
                }
            }
            try seen.put(page, {});
        }

        const mid_idx: usize = book.items().len / 2;
        count1 += book.items()[mid_idx];
    }

    // https://github.com/Darkness4/advent-of-code-2024/blob/main/src/day05.zig

    // Day 05 Star 2.
    var count2: u32 = 0;
    for (books.items) |book| {
        var is_valid = false;
        var was_invalid = false;
        redo: while (!is_valid) {
            is_valid = true;

            for (book.items(), 0..) |pi, i| {
                for (book.items()[0..i], 0..) |pj, j| {
                    if (rules.get(pi)) |post_pages| {
                        var is_in = false;
                        for (post_pages.items) |post_page| {
                            if (post_page == pj) {
                                is_in = true;
                                break;
                            }
                        }

                        if (is_in) {
                            is_valid = false;
                            was_invalid = true;

                            std.mem.copyForwards(u32, book.items()[j..], book.items()[j + 1 ..]);
                            book.items()[book.items().len - 1] = pj;
                            continue :redo;
                        }
                    }
                }
            }

            if (was_invalid) {
                const mid_idx: usize = book.items().len / 2;
                count2 += book.items()[mid_idx];
            }
        }
    }

    const stdout_writer = std.io.getStdOut().writer();
    var stdout_bw = std.io.bufferedWriter(stdout_writer);
    const stdout = stdout_bw.writer();

    try stdout.print("Day 05 Star 1 solution = {}\n", .{count1});
    try stdout.print("Day 05 Star 2 solution = {}\n", .{count2});

    try stdout_bw.flush();
}
