const std = @import("std");

const InstructionKind = enum {
    Do,
    Dont,
    From,
    How,
    Mul,
    Select,
    Who,
    What,
    When,
    Where,
    Why,
};

const ArgPair = struct {
    a: u32,
    b: u32,
};

const Token = union(InstructionKind) {
    Do: void,
    Dont: void,
    From: ?ArgPair,
    How: ?ArgPair,
    Mul: ArgPair,
    Select: ?ArgPair,
    Who: ?ArgPair,
    What: ?ArgPair,
    When: ?ArgPair,
    Where: ?ArgPair,
    Why: ?ArgPair,
};

fn next(input: []const u8, idx: *usize) !void {
    idx.* += 1;
    if (idx.* >= input.len) return error.EOF;
}

fn tokenize_u32(input: []const u8, idx: *usize) !u32 {
    var value: u32 = 0;
    while ('0' <= input[idx.*] and input[idx.*] <= '9') {
        value = 10 * value + (input[idx.*] - '0');
        try next(input, idx);
    }
    return value;
}

fn tokenize_argpair(input: []const u8, idx: *usize) !?ArgPair {
    if (input[idx.*] == ')') return null;

    const a = try tokenize_u32(input, idx);

    if (input[idx.*] != ',') return error.InvalidToken;

    try next(input, idx);
    const b = try tokenize_u32(input, idx);

    if (input[idx.*] != ')') return error.InvalidToken;

    return ArgPair{ .a = a, .b = b };
}

// Used https://github.com/Darkness4/advent-of-code-2024/blob/main/src/day03.zig as reference for how to do the loop-switch parsing strategy in Zig
fn tokenize(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(Token) {
    var tokens = try std.ArrayList(Token).initCapacity(allocator, 1000);

    var idx: usize = 0;
    scan: while (idx < input.len) : (idx += 1) {
        redo: switch (input[idx]) {
            'd' => {
                next(input, &idx) catch break :scan;
                if (input[idx] != 'o') break :redo;

                next(input, &idx) catch break :scan;
                if (input[idx] == '(') {
                    next(input, &idx) catch break :scan;
                    if (input[idx] == ')') try tokens.append(Token.Do);
                } else if (input[idx] == 'n') {
                    for ("'t()") |n| {
                        next(input, &idx) catch break :scan;
                        if (input[idx] != n) break :redo;
                    }
                    try tokens.append(Token.Dont);
                }
            },
            'f' => {
                for ("rom(") |n| {
                    next(input, &idx) catch break :scan;
                    if (input[idx] != n) break :redo;
                }

                next(input, &idx) catch break :scan;
                const args = tokenize_argpair(input, &idx) catch break :redo;

                try tokens.append(Token{ .From = args });
            },
            'h' => {
                for ("ow(") |n| {
                    next(input, &idx) catch break :scan;
                    if (input[idx] != n) break :redo;
                }

                next(input, &idx) catch break :scan;
                const args = tokenize_argpair(input, &idx) catch break :redo;

                try tokens.append(Token{ .How = args });
            },
            'm' => {
                for ("ul(") |n| {
                    next(input, &idx) catch break :scan;
                    if (input[idx] != n) break :redo;
                }

                next(input, &idx) catch break :scan;
                const args = tokenize_argpair(input, &idx) catch break :redo;

                if (args != null)
                    try tokens.append(Token{ .Mul = args.? });
            },
            's' => {
                for ("elect(") |n| {
                    next(input, &idx) catch break :scan;
                    if (input[idx] != n) break :redo;
                }

                next(input, &idx) catch break :scan;
                const args = tokenize_argpair(input, &idx) catch break :redo;

                try tokens.append(Token{ .Select = args });
            },
            'w' => {
                next(input, &idx) catch break :scan;
                if (input[idx] != 'h') break :redo;

                next(input, &idx) catch break :scan;
                switch (input[idx]) {
                    'o' => {
                        next(input, &idx) catch break :scan;
                        if (input[idx] != '(') break :redo;

                        next(input, &idx) catch break :scan;
                        const args = tokenize_argpair(input, &idx) catch break :redo;

                        try tokens.append(Token{ .Who = args });
                    },
                    'a' => {
                        next(input, &idx) catch break :scan;
                        if (input[idx] != 't') break :redo;

                        next(input, &idx) catch break :scan;
                        if (input[idx] != '(') break :redo;

                        next(input, &idx) catch break :scan;
                        const args = tokenize_argpair(input, &idx) catch break :redo;

                        try tokens.append(Token{ .What = args });
                    },
                    'e' => {
                        next(input, &idx) catch break :scan;
                        if (input[idx] == 'n') {
                            next(input, &idx) catch break :scan;
                            if (input[idx] != '(') break :redo;

                            next(input, &idx) catch break :scan;
                            const args = tokenize_argpair(input, &idx) catch break :redo;

                            try tokens.append(Token{ .When = args });
                        } else if (input[idx] == 'r') {
                            next(input, &idx) catch break :scan;
                            if (input[idx] != 'e') break :redo;

                            next(input, &idx) catch break :scan;
                            if (input[idx] != '(') break :redo;

                            next(input, &idx) catch break :scan;
                            const args = tokenize_argpair(input, &idx) catch break :redo;

                            try tokens.append(Token{ .Where = args });
                        }
                    },
                    'y' => {
                        next(input, &idx) catch break :scan;
                        if (input[idx] != '(') break :redo;

                        next(input, &idx) catch break :scan;
                        const args = tokenize_argpair(input, &idx) catch break :redo;

                        try tokens.append(Token{ .Why = args });
                    },
                    else => {},
                }
            },
            else => {},
        }
    }

    return tokens;
}

pub fn main() !void {
    const input = @embedFile("input.txt");

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const allocator = gpa.allocator();

    const tokens = try tokenize(allocator, input);
    defer tokens.deinit();

    // Day 03 Star 1.
    var sum1: u32 = 0;
    for (tokens.items) |token| {
        switch (token) {
            .Mul => |args| {
                sum1 += args.a * args.b;
            },
            else => {},
        }
    }

    // Day 03 Star 2.
    var sum2: u32 = 0;
    var is_in_do_block: bool = true;
    for (tokens.items) |token| {
        switch (token) {
            .Mul => |args| {
                if (is_in_do_block)
                    sum2 += args.a * args.b;
            },
            .Do => {
                is_in_do_block = true;
            },
            .Dont => {
                is_in_do_block = false;
            },
            else => {},
        }
    }

    const stdout = std.io.getStdOut().writer();
    try stdout.print("Day 03 Star 1 solution = {}\n", .{sum1});
    try stdout.print("Day 03 Star 2 solution = {}\n", .{sum2});
}
