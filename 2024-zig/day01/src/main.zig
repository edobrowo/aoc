const std = @import("std");

fn parse(allocator: std.mem.Allocator, path: []const u8) !struct { std.ArrayList(u32), std.ArrayList(u32) } {
    // Open file and set up reader.
    const input = try std.fs.cwd().openFile(path, .{});
    defer input.close();

    var buf_reader = std.io.bufferedReader(input.reader());
    const in_stream = buf_reader.reader();

    var buf: [1024]u8 = undefined;

    // Day 01: left and right ID lists.
    var left = try std.ArrayList(u32).initCapacity(allocator, 10);
    var right = try std.ArrayList(u32).initCapacity(allocator, 10);

    while (true) {
        // "Split by newlines" - fill a fixed buffer with the line.
        var fbs = std.io.fixedBufferStream(&buf);
        const result = in_stream.streamUntilDelimiter(fbs.writer(), '\n', fbs.buffer.len);

        if (result) |_| {
            const output = fbs.getWritten();

            // Split each line into left and right IDs.
            var it = std.mem.splitSequence(u8, output, "   ");
            const left_str = it.next() orelse return error.MissingLeft;
            const right_str = it.next() orelse return error.MissingRight;

            const left_id = try std.fmt.parseInt(u32, left_str, 10);
            const right_id = try std.fmt.parseInt(u32, right_str, 10);

            try left.append(left_id);
            try right.append(right_id);
        } else |err| {
            if (err == error.EndOfStream) {
                break;
            } else {
                return err;
            }
        }
    }

    return .{ left, right };
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const allocator = gpa.allocator();

    const stdout = std.io.getStdOut().writer();

    const path = "input.txt";
    const result = try parse(allocator, path[0..path.len]);

    const left = result[0];
    defer left.deinit();

    const right = result[1];
    defer right.deinit();

    // Day 1 Star 1.
    var total_distance: u32 = 0;

    std.mem.sort(u32, left.items, void{}, comptime std.sort.asc(u32));
    std.mem.sort(u32, right.items, void{}, comptime std.sort.asc(u32));

    for (left.items, right.items) |l, r| {
        const distance = if (l > r) l - r else r - l;
        total_distance += distance;
    }

    try stdout.print("Day 01 Star 1 solution = {}\n", .{total_distance});

    // Day 1 Star 2.
    var total_score: u32 = 0;

    var hashmap = std.AutoHashMap(u32, u32).init(allocator);
    defer hashmap.deinit();

    for (right.items) |r| {
        const entry = try hashmap.getOrPut(r);
        if (entry.found_existing) {
            entry.value_ptr.* += 1;
        } else {
            entry.value_ptr.* = 1;
        }
    }

    for (left.items) |l| {
        const count = hashmap.get(l) orelse 0;
        total_score += l * count;
    }

    try stdout.print("Day 01 Star 2 solution = {}\n", .{total_score});
}
