<script setup>
const version = __APP_VERSION__
</script>

<template>
  <div class="header">
    <div class="image"><img src="../assets/img/icon.png" /></div>
    <div class="text">
      <div class="title">無人頒布システム</div>
      <div class="version">Version {{ version }}</div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.header {
  padding-left: 20px;
  padding-top: 20px;
  overflow: hidden;

  .image,
  text {
    float: left;
  }

  .image {
    padding-right: 12px;

    img {
      width: 60px;
      height: 60px;
    }
  }

  .text {
    padding: 4px;

    .title {
      font-size: 20px;
    }

    .version {
      color: #9e9e9e;
    }
  }
}
</style>
