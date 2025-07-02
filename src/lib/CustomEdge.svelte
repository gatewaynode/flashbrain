<script>
  import { getSmoothStepPath } from '@xyflow/svelte';
  
  export let id;
  export let sourceX;
  export let sourceY;
  export let targetX;
  export let targetY;
  export let sourcePosition;
  export let targetPosition;
  export let style = '';
  export let markerEnd;
  export let data;
  
  $: [edgePath] = getSmoothStepPath({
    sourceX,
    sourceY,
    sourcePosition,
    targetX,
    targetY,
    targetPosition,
  });
</script>

<path
  id={id}
  style="pointer-events: all;"
  class="react-flow__edge-path"
  d={edgePath}
  marker-end={markerEnd}
  stroke="#ff0000"
  stroke-width="4"
  stroke-dasharray="10,5"
  fill="none"
>
  <animate
    attributeName="stroke-dashoffset"
    values="0;-15"
    dur="1s"
    repeatCount="indefinite"
  />
</path> 