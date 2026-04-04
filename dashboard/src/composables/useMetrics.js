import { ref, onMounted, onUnmounted } from 'vue'

const API_BASE = import.meta.env.VITE_API_BASE || ''

export function useMetrics() {
  const metrics = ref(null)
  const history = ref([])
  const connected = ref(false)
  let eventSource = null

  function applyData(data) {
    metrics.value = {
      system: data.system,
      cores: data.cores,
      disks: data.disks,
    }
    history.value = data.history?.cluster || []
  }

  async function fetchInitial() {
    const res = await fetch(`${API_BASE}/api/metrics`)
    const data = await res.json()
    if (!res.ok) throw new Error(data.error ?? 'Failed to fetch metrics')
    applyData(data)
  }

  function connectSSE() {
    eventSource = new EventSource(`${API_BASE}/api/metrics/stream`)

    eventSource.onopen = () => {
      connected.value = true
    }

    eventSource.onmessage = (event) => {
      applyData(JSON.parse(event.data))
    }

    eventSource.onerror = () => {
      connected.value = false
    }
  }

  onMounted(async () => {
    await fetchInitial()
    connectSSE()
  })

  onUnmounted(() => {
    if (eventSource) {
      eventSource.close()
    }
  })

  return { metrics, history, connected }
}
