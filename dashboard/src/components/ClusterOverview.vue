<script setup>
import { computed } from 'vue'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Progress } from '@/components/ui/progress'
import { ChartContainer, ChartTooltip, ChartTooltipContent } from '@/components/ui/chart'
import { VisXYContainer, VisArea, VisAxis } from '@unovis/vue'

const props = defineProps({
  system: { type: Array, required: true },
  history: { type: Array, required: true },
})

const totalCores = computed(() =>
  props.system.reduce((sum, n) => sum + n.cpu_core_count, 0)
)

const avgCpu = computed(() => {
  if (!props.system.length) return 0
  return props.system.reduce((sum, n) => sum + n.cpu_usage_percent, 0) / props.system.length
})

const totalMem = computed(() =>
  props.system.reduce((sum, n) => sum + n.mem_total_bytes, 0)
)

const usedMem = computed(() =>
  props.system.reduce((sum, n) => sum + n.mem_used_bytes, 0)
)

const memPercent = computed(() =>
  totalMem.value > 0 ? (usedMem.value / totalMem.value) * 100 : 0
)

function formatBytes(bytes) {
  if (bytes === 0) return '0 B'
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(Math.abs(bytes)) / Math.log(1024))
  return (bytes / Math.pow(1024, i)).toFixed(1) + ' ' + units[i]
}

const cpuChartConfig = {
  cpu: { label: 'CPU %', color: 'var(--chart-1)' },
}

const memChartConfig = {
  mem: { label: 'Memory', color: 'var(--chart-2)' },
}

const cpuX = (d) => d.timestamp
const cpuY = (d) => d.cpu_usage_percent
const memY = (d) =>
  d.mem_total_bytes > 0 ? (d.mem_used_bytes / d.mem_total_bytes) * 100 : 0

const yDomain = [0, 100]

// Key changes whenever new history arrives, forcing Unovis to re-render
const historyKey = computed(() => props.history.at(-1)?.timestamp ?? 0)

function formatTime(ms) {
  return new Date(ms).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
}
</script>

<template>
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <!-- CPU Summary -->
    <Card>
      <CardHeader class="pb-2">
        <CardTitle class="text-sm font-medium text-muted-foreground">
          Cluster CPU
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div class="flex items-baseline gap-2 mb-2">
          <span class="text-2xl font-bold">{{ avgCpu.toFixed(1) }}%</span>
          <span class="text-sm text-muted-foreground">
            across {{ totalCores }} cores ({{ system.length }} nodes)
          </span>
        </div>
        <Progress :model-value="avgCpu" class="h-3" />

        <div class="mt-4 h-[150px]">
          <ChartContainer :config="cpuChartConfig">
            <VisXYContainer :key="historyKey" :data="history" :y-domain="yDomain">
              <VisArea :x="cpuX" :y="cpuY" color="var(--chart-1)" :opacity="0.3" />
              <VisAxis type="x" :tick-format="formatTime" :num-ticks="5" />
              <VisAxis type="y" :tick-format="(v) => v.toFixed(0) + '%'" :num-ticks="4" />
            </VisXYContainer>
          </ChartContainer>
        </div>
      </CardContent>
    </Card>

    <!-- Memory Summary -->
    <Card>
      <CardHeader class="pb-2">
        <CardTitle class="text-sm font-medium text-muted-foreground">
          Cluster Memory
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div class="flex items-baseline gap-2 mb-2">
          <span class="text-2xl font-bold">{{ memPercent.toFixed(1) }}%</span>
          <span class="text-sm text-muted-foreground">
            {{ formatBytes(usedMem) }} / {{ formatBytes(totalMem) }}
          </span>
        </div>
        <Progress :model-value="memPercent" class="h-3" />

        <div class="mt-4 h-[150px]">
          <ChartContainer :config="memChartConfig">
            <VisXYContainer :key="historyKey" :data="history" :y-domain="yDomain">
              <VisArea :x="cpuX" :y="memY" color="var(--chart-2)" :opacity="0.3" />
              <VisAxis type="x" :tick-format="formatTime" :num-ticks="5" />
              <VisAxis type="y" :tick-format="(v) => v.toFixed(0) + '%'" :num-ticks="4" />
            </VisXYContainer>
          </ChartContainer>
        </div>
      </CardContent>
    </Card>
  </div>
</template>
