<script setup>
import { computed } from 'vue'
import { useMetrics } from './composables/useMetrics.js'
import ClusterOverview from './components/ClusterOverview.vue'
import NodeCard from './components/NodeCard.vue'

const OFFLINE_THRESHOLD_MS = 60 * 1000

const { metrics, history, connected } = useMetrics()

const nodes = computed(() => {
  if (!metrics.value) return []
  const nodeNames = [...new Set(metrics.value.system.map((s) => s.node_name))]
  return nodeNames
    .map((name) => {
      const system = metrics.value.system.find((s) => s.node_name === name)
      const online = Date.now() - system.timestamp < OFFLINE_THRESHOLD_MS
      return {
        system,
        cores: metrics.value.cores
          .filter((c) => c.node_name === name)
          .sort((a, b) => a.core_id - b.core_id),
        online,
      }
    })
    .sort((a, b) => b.online - a.online)
})

const onlineNodes = computed(() => nodes.value.filter((n) => n.online))

const mostLoadedName = computed(() => {
  if (!onlineNodes.value.length) return null
  return onlineNodes.value.reduce((a, b) =>
    a.system.cpu_usage_percent > b.system.cpu_usage_percent ? a : b
  ).system.node_name
})
</script>

<template>
  <div class="dark min-h-screen bg-background text-foreground">
    <div class="max-w-6xl mx-auto p-6 space-y-6">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-bold tracking-tight">Node Metrics</h1>
          <p class="text-xs text-muted-foreground mt-0.5">
            {{ onlineNodes.length }} online
            <span v-if="nodes.length !== onlineNodes.length">
              / {{ nodes.length }} total
            </span>
          </p>
        </div>
        <span
          class="text-xs px-2.5 py-1 rounded-full"
          :class="connected
            ? 'bg-emerald-500/10 text-emerald-400'
            : 'bg-muted text-muted-foreground'"
        >
          {{ connected ? 'Live' : 'Connecting...' }}
        </span>
      </div>

      <div v-if="!metrics" class="text-center text-muted-foreground py-12">
        Loading metrics...
      </div>

      <template v-else>
        <!-- Cluster Overview (online nodes only) -->
        <ClusterOverview :system="onlineNodes.map((n) => n.system)" :history="history" />

        <!-- Per-Node -->
        <div>
          <h2 class="text-lg font-semibold mb-3">Nodes</h2>
          <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
            <NodeCard
              v-for="node in nodes"
              :key="node.system.node_name"
              :node="node.system"
              :cores="node.cores"
              :online="node.online"
              :most-loaded="node.system.node_name === mostLoadedName"
            />
          </div>
        </div>
      </template>
    </div>
  </div>
</template>
