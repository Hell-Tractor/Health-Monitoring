<template>
  <footer class="footer">
    <div class="container-fluid d-flex flex-wrap justify-content-between">
      <nav>
        <ul>
          <li>
            <router-link :to="{ path: '/home' }">Home</router-link>
          </li>
        </ul>
      </nav>
      <div class="copyright d-flex flex-wrap">
        吴佳熙 & 陈一鸣 团队出品
      </div>
    </div>
  </footer>
</template>
<script>
export default {};
</script>
<style></style>
