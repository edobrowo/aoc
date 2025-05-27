const std = @import("std");

const print = std.debug.print;

const Ordering = enum { Increasing, Decreasing, Equal };

fn getOrder(a: u32, b: u32) Ordering {
    return if (b > a)
        Ordering.Increasing
    else if (a > b)
        Ordering.Decreasing
    else
        Ordering.Equal;
}

pub fn main() !void {
    const file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var bw = std.io.bufferedReader(file.reader());
    const in_stream = bw.reader();

    var buf: [1024]u8 = undefined;

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const allocator = gpa.allocator();

    var reports = try std.ArrayList(std.ArrayList(u32)).initCapacity(allocator, 100);
    defer reports.deinit();

    while (true) {
        var fbs = std.io.fixedBufferStream(&buf);
        const result = in_stream.streamUntilDelimiter(fbs.writer(), '\n', fbs.buffer.len);

        if (result) |_| {
            var report = try std.ArrayList(u32).initCapacity(allocator, 10);

            const output = fbs.getWritten();

            var it = std.mem.splitScalar(u8, output, ' ');
            while (it.next()) |entry| {
                const level: u32 = try std.fmt.parseInt(u32, entry, 10);
                try report.append(level);
            }

            try reports.append(report);
        } else |err| {
            if (err == error.EndOfStream) {
                break;
            } else {
                return err;
            }
        }
    }

    // Day 02 Star 1.
    var safe1: u32 = 0;
    for (reports.items) |report| {
        const exp_order = getOrder(report.items[0], report.items[1]);
        var is_safe = true;

        var prev: u32 = report.items[0];
        for (report.items[1..]) |level| {
            const diff = @abs(@as(i32, @intCast(level)) - @as(i32, @intCast(prev)));

            const order = getOrder(prev, level);

            if ((diff == 0) or (diff > 3) or (order != exp_order))
                is_safe = false;

            prev = level;
        }

        if (is_safe) safe1 += 1;
    }

    // Day 02 Star 2.
    var safe2: u32 = 0;
    for (reports.items) |report| {
        for (0..report.items.len) |skip| {
            var report_copy = try std.ArrayList(u32).initCapacity(allocator, 10);
            defer report_copy.deinit();

            for (0..report.items.len) |i| {
                if (i != skip)
                    try report_copy.append(report.items[i]);
            }

            const exp_order = getOrder(report_copy.items[0], report_copy.items[1]);
            var is_safe = true;

            var prev: u32 = report_copy.items[0];
            for (report_copy.items[1..]) |level| {
                const diff = @abs(@as(i32, @intCast(level)) - @as(i32, @intCast(prev)));

                const order = getOrder(prev, level);

                if ((diff == 0) or (diff > 3) or (order != exp_order))
                    is_safe = false;

                prev = level;
            }

            if (is_safe) {
                safe2 += 1;
                break;
            }
        }
    }

    const stdout = std.io.getStdOut().writer();
    try stdout.print("Day 02 Star 1 solution = {}\n", .{safe1});
    try stdout.print("Day 02 Star 2 solution = {}\n", .{safe2});

    for (reports.items) |report| {
        report.deinit();
    }
}
