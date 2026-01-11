<template>
  <div class="header">
    <div class="image"><img src="../assets/img/icon.png" /></div>
    <div class="text">無人頒布システム</div>
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
    padding: 12px;
    font-size: 20px;
  }
}
</style>
