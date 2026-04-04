<script setup>
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Progress } from '@/components/ui/progress'

defineProps({
  node: { type: Object, required: true },
  cores: { type: Array, required: true },
  disks: { type: Array, required: true },
})

function formatBytes(bytes) {
  if (bytes === 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(Math.abs(bytes)) / Math.log(1024))
  return (bytes / Math.pow(1024, i)).toFixed(1) + ' ' + units[i]
}
</script>

<template>
  <Card>
    <CardHeader class="pb-3">
      <CardTitle>{{ node.node_name }}</CardTitle>
    </CardHeader>
    <CardContent class="space-y-4">
      <!-- CPU -->
      <div>
        <div class="flex items-center justify-between mb-1">
          <span class="text-sm font-medium text-muted-foreground">CPU</span>
          <span class="text-sm font-mono">{{ node.cpu_usage_percent.toFixed(1) }}%</span>
        </div>
        <Progress :model-value="node.cpu_usage_percent" class="h-2" />
        <div class="grid grid-cols-[repeat(auto-fill,minmax(60px,1fr))] gap-1 mt-2">
          <div
            v-for="core in cores"
            :key="core.core_id"
            class="flex items-center gap-1"
          >
            <Progress :model-value="core.usage_percent" class="h-1 flex-1" />
            <span class="text-[10px] text-muted-foreground tabular-nums w-7 text-right">
              {{ core.usage_percent.toFixed(0) }}%
            </span>
          </div>
        </div>
      </div>

      <!-- Memory -->
      <div>
        <div class="flex items-center justify-between mb-1">
          <span class="text-sm font-medium text-muted-foreground">Memory</span>
          <span class="text-sm font-mono">
            {{ formatBytes(node.mem_used_bytes) }} / {{ formatBytes(node.mem_total_bytes) }}
          </span>
        </div>
        <Progress
          :model-value="node.mem_total_bytes > 0 ? (node.mem_used_bytes / node.mem_total_bytes) * 100 : 0"
          class="h-2"
        />
      </div>

      <!-- Swap -->
      <div v-if="node.swap_total_bytes > 0">
        <div class="flex items-center justify-between mb-1">
          <span class="text-sm font-medium text-muted-foreground">Swap</span>
          <span class="text-sm font-mono">
            {{ formatBytes(node.swap_used_bytes) }} / {{ formatBytes(node.swap_total_bytes) }}
          </span>
        </div>
        <Progress
          :model-value="(node.swap_used_bytes / node.swap_total_bytes) * 100"
          class="h-2"
        />
      </div>

      <!-- Disks -->
      <div v-if="disks.length">
        <span class="text-sm font-medium text-muted-foreground">Disks</span>
        <div class="space-y-2 mt-1">
          <div v-for="disk in disks" :key="disk.name">
            <div class="flex items-center justify-between mb-1">
              <span class="text-xs text-muted-foreground" :title="disk.mount_point">
                {{ disk.name }}
              </span>
              <span class="text-xs font-mono text-muted-foreground">
                {{ formatBytes(disk.used_bytes) }} / {{ formatBytes(disk.total_bytes) }}
              </span>
            </div>
            <Progress :model-value="disk.usage_percent" class="h-1.5" />
          </div>
        </div>
      </div>
    </CardContent>
  </Card>
</template>
