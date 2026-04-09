<script setup>
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Progress } from '@/components/ui/progress'

defineProps({
  node: { type: Object, required: true },
  cores: { type: Array, required: true },
  online: { type: Boolean, default: true },
  mostLoaded: { type: Boolean, default: false },
})

function formatBytes(bytes) {
  if (bytes === 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(Math.abs(bytes)) / Math.log(1024))
  return (bytes / Math.pow(1024, i)).toFixed(1) + ' ' + units[i]
}

function memPressure(node) {
  if (node.mem_total_bytes === 0) return false
  return node.mem_available_bytes / node.mem_total_bytes < 0.1
}

function timeAgo(ms) {
  const diff = Date.now() - ms
  if (diff < 60_000) return `${Math.floor(diff / 1000)}s ago`
  if (diff < 3_600_000) return `${Math.floor(diff / 60_000)}m ago`
  return `${Math.floor(diff / 3_600_000)}h ago`
}
</script>

<template>
  <Card :class="[!online ? 'opacity-60' : '', mostLoaded && online ? 'ring-1 ring-amber-500/50' : '']">
    <CardHeader class="pb-3">
      <div class="flex items-center justify-between">
        <CardTitle>{{ node.node_name }}</CardTitle>
        <div class="flex items-center gap-2">
          <span class="text-xs text-muted-foreground">{{ timeAgo(node.timestamp) }}</span>
          <span
            class="text-xs px-2 py-0.5 rounded-full"
            :class="online
              ? 'bg-emerald-500/10 text-emerald-400'
              : 'bg-muted text-muted-foreground'"
          >
            {{ online ? 'Online' : 'Offline' }}
          </span>
        </div>
      </div>
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
          <div class="flex items-center gap-2">
            <span class="text-sm font-medium text-muted-foreground">Memory</span>
            <span v-if="memPressure(node)" class="text-xs px-1.5 py-0.5 rounded-full bg-red-500/10 text-red-400">
              pressure
            </span>
          </div>
          <span class="text-sm font-mono">
            {{ formatBytes(node.mem_used_bytes) }} / {{ formatBytes(node.mem_total_bytes) }}
          </span>
        </div>
        <Progress
          :model-value="node.mem_total_bytes > 0 ? (node.mem_used_bytes / node.mem_total_bytes) * 100 : 0"
          class="h-2"
        />
      </div>
    </CardContent>
  </Card>
</template>
