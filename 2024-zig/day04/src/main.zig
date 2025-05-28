const std = @import("std");

fn get(grid: std.ArrayList(u8), w: usize, x: usize, y: usize) u8 {
    return grid.items[y * w + x];
}

pub fn main() !void {
    const file = try std.fs.cwd().openFile("input.txt", .{});
    defer file.close();

    var bufr = std.io.bufferedReader(file.reader());
    const in_stream = bufr.reader();

    var buf: [1024]u8 = undefined;

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const allocator = gpa.allocator();

    var grid = try std.ArrayList(u8).initCapacity(allocator, 1024);
    defer grid.deinit();

    var width: usize = 0;
    var height: usize = 0;

    while (true) {
        var fbs = std.io.fixedBufferStream(&buf);
        const result = in_stream.streamUntilDelimiter(fbs.writer(), '\n', fbs.buffer.len);

        if (result) |_| {
            const written = fbs.getWritten();

            if (width == 0) width = written.len;
            height += 1;

            try grid.appendSlice(written[0..written.len]);
        } else |err| {
            if (err == error.EndOfStream) {
                break;
            } else {
                return err;
            }
        }
    }

    const offsets = [8][2]i32{
        .{ -1, -1 },
        .{ 0, -1 },
        .{ 1, -1 },
        .{ -1, 0 },
        .{ 1, 0 },
        .{ -1, 1 },
        .{ 0, 1 },
        .{ 1, 1 },
    };

    const MAS = "MAS";

    // Day 04 Star 1.
    var count1: u32 = 0;
    for (0..height) |yu| {
        nextchar: for (0..width) |xu| {
            const c = get(grid, width, xu, yu);

            const x = @as(i32, @truncate(@as(i128, xu)));
            const y = @as(i32, @truncate(@as(i128, yu)));

            if (c != 'X')
                continue :nextchar;

            nextoffset: for (offsets) |offset| {
                const x_end = x + 3 * offset[0];
                const y_end = y + 3 * offset[1];

                if (x_end < 0 or
                    @as(i32, @truncate(@as(i128, width))) <= x_end or
                    y_end < 0 or
                    @as(i32, @truncate(@as(i128, height))) <= y_end)
                {
                    continue :nextoffset;
                }

                for (1..4) |u| {
                    const i = @as(i32, @truncate(@as(i128, u)));
                    const xp = x + i * offset[0];
                    const yp = y + i * offset[1];
                    const xup = @as(usize, @intCast(xp));
                    const yup = @as(usize, @intCast(yp));
                    const cp = get(grid, width, xup, yup);
                    if (cp != MAS[u - 1])
                        continue :nextoffset;
                }

                count1 += 1;
            }
        }
    }

    // Day 04 Star 2.
    var count2: u32 = 0;
    for (1..height - 1) |y| {
        nextchar: for (1..width - 1) |x| {
            if (get(grid, width, x, y) != 'A')
                continue :nextchar;

            const tl = get(grid, width, x - 1, y - 1);
            const tr = get(grid, width, x + 1, y - 1);
            const bl = get(grid, width, x - 1, y + 1);
            const br = get(grid, width, x + 1, y + 1);

            if (((tl == 'M' and br == 'S') or (tl == 'S' and br == 'M')) and
                ((tr == 'M' and bl == 'S') or (tr == 'S' and bl == 'M')))
                count2 += 1;
        }
    }

    const stdout_writer = std.io.getStdOut().writer();
    var stdout_bw = std.io.bufferedWriter(stdout_writer);
    const stdout = stdout_bw.writer();

    try stdout.print("Day 04 Star 1 Solution = {}\n", .{count1});
    try stdout.print("Day 04 Star 2 Solution = {}\n", .{count2});

    try stdout_bw.flush();
}
