// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
#include <vmlinux.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 10240);
    __type(key, u64);
    __type(value, u64);
} counts SEC(".maps");

struct {
    __uint(type, BPF_MAP_TYPE_STACK_TRACE);
    __uint(key_size, sizeof(u32));
    __uint(value_size, PERF_MAX_STACK_DEPTH * sizeof(u64));
    __uint(max_entries, 10240);
} stacks SEC(".maps");

SEC("perf_event")
int oncpu(struct bpf_perf_event_data *ctx)
{
    u64 pid_tgid = bpf_get_current_pid_tgid();
    u32 pid = pid_tgid >> 32;
    u32 user_id = bpf_get_stackid(ctx, &stacks, BPF_F_USER_STACK);
    u32 kern_id = bpf_get_stackid(ctx, &stacks, 0);
    u64 key = ((u64)user_id << 32) | kern_id;
    u64 *val = bpf_map_lookup_elem(&counts, &key);
    u64 one = 1;
    if (val) {
        __sync_fetch_and_add(val, one);
    } else {
        bpf_map_update_elem(&counts, &key, &one, BPF_ANY);
    }
    return 0;
}

char LICENSE[] SEC("license") = "Dual BSD/GPL";
