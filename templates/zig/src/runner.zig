const std = @import("std");
const solver = @import("main.zig");

pub fn main() !void {
    const variant_and_run_type = switch (std.os.argv.len) {
        1 => .{ Variant.both, RunType.run },
        2 => {
            const arg = std.ascii.upperString(std.os.argv[1], std.os.argv[1]);
            if (std.mem.eql(u8, arg, "both")) .{ Variant.both, RunType.run } else if (std.mem.eql(u8, arg, "a")) .{ Variant.a, RunType.run } else if (std.mem.eql(u8, arg, "b")) .{ Variant.b, RunType.run } else if (std.mem.eql(u8, arg, "samples")) .{ Variant.both, RunType.samples } else if (std.mem.eql(u8, arg, "samples-a")) .{ Variant.a, RunType.samples } else if (std.mem.eql(u8, arg, "samples-b")) .{ Variant.b, RunType.samples } else @panic("invalid parameter");
        },
        _ => @panic("can only have one argument"),
    };

    const variant = variant_and_run_type[0];
    const run_type = variant_and_run_type[1];

    switch (run_type) {
        RunType.run => run(variant),
        RunType.samples => runSamples(variant),
    }
}

const RunType = enum { run, samples };

const Variant = enum {
    a,
    b,
    both,
};

fn run(variant: Variant) !void {
    const input = @embedFile("../assets/input");

    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    switch (variant) {
        Variant.a => {
            const result = try solver.solveA(input);
            try stdout.print("{}\n", .{result});
            try bw.flush();
        },
        Variant.b => {
            const result = try solver.solveB(input);
            try stdout.print("{}\n", .{result});
            try bw.flush();
        },
        Variant.both => {
            try stdout.print("Solution A:\n{}\n\n", .{try solver.solveA(input)});
            try bw.flush();

            try stdout.print("Solution B:\n{}\n", .{try solver.solveA(input)});
            try bw.flush();
        },
    }
}

fn runSamples(variant: Variant) !void {
    switch (variant) {
        Variant.a => runSamplesA(),
        Variant.b => runSamplesB(),
        Variant.both => {
            runSamplesA();
            runSamplesB();
        },
    }
}

fn runSamplesA() !void {
    const samples_dir = try std.fs.cwd().openDir(
        "assets/examples/a",
        .{ .iterate = true },
    );

    defer samples_dir.close();

    var samples_iter = samples_dir.iterate();

    while (try samples_iter.next()) |sample_entry| {
        const sample = try samples_dir.openDir(sample_entry.name, .{});
        const input = try sample.openFile("input", .{});
        const expected = try sample.openFile("expected", .{});

        const result = try solver.solveA(input);
        std.debug.assert(result == expected);
    }
}

fn runSamplesB() !void {
    const samples_dir = try std.fs.cwd().openDir(
        "assets/examples/b",
        .{ .iterate = true },
    );

    defer samples_dir.close();

    var samples_iter = samples_dir.iterate();

    while (try samples_iter.next()) |sample_entry| {
        const sample = try samples_dir.openDir(sample_entry.name, .{});
        const input = try sample.openFile("input", .{});
        const expected = try sample.openFile("expected", .{});

        const result = try solver.solveB(input);
        std.debug.assert(result == expected);
    }
}
