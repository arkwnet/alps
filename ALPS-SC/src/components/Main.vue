<script setup>
import { ref, onMounted } from 'vue'

const items = ref([
  { id: 'engineer-cn-v3', name: 'ｴﾝｼﾞﾆｱの中国語入門 第3版', price: 300 },
  { id: 'iroiro-android', name: 'ｲﾛｲﾛ・ｱﾝﾄﾞﾛｲﾄﾞ!!', price: 500 },
  { id: 'abnormal-distribution', name: '異常頒布', price: 300 },
  { id: 'trainsim-cookbook', name: 'ﾄﾚｲﾝｼﾐｭﾚｰﾀｸｯｸﾌﾞｯｸ', price: 500 },
])
const isMain = ref(true)
const isFinish = ref(false)
const isCover = ref(true)
const cart = ref([])
const total = ref(0)

onMounted(() => {
  isCover.value = false
})

const getImageUrl = (id) => {
  return new URL(`../assets/img/${id}.png`, import.meta.url).href
}

const onChange = () => {
  let subtotal = 0
  for (let i = 0; i < cart.value.length; i++) {
    subtotal += cart.value[i].price
  }
  total.value = subtotal
}

const onSubmit = async () => {
  isCover.value = true
  let jsonArray = new Array()
  for (let i = 0; i < cart.value.length; i++) {
    jsonArray.push({
      name: cart.value[i].name,
      price: '' + cart.value[i].price,
      quantity: '1',
    })
  }
  let json = {
    id: 'SELF',
    items: jsonArray,
    total: '' + total.value,
    payment: '現金 (無人頒布)',
    cash: '' + total.value,
    change: '0',
  }
  try {
    const response = await fetch('./backend/checkout.php', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(json),
    })
    const result = await response.json()
    if (result.status != 0) {
      isMain.value = false
      isFinish.value = true
    } else {
      alert(
        '通信エラーが発生しました。お手数ですが、もう一度お試しください。エラーコード: ' +
          result.status,
      )
    }
    isCover.value = false
  } catch (error) {
    isCover.value = false
    alert('通信エラーが発生しました。お手数ですが、もう一度お試しください。エラー内容: ' + error)
  }
}
</script>

<template>
  <div class="main">
    <p>
      無人頒布システムへようこそ!<br />
      説明文をよく読みながら操作してください。
    </p>
    <h1>Step. 1</h1>
    <p>
      購入する頒布物のチェックボックスにチェックを入れてください。<br />
      ※ ここに表示されていない頒布物は無人頒布で購入できません。有人頒布の再開をお待ちください。<br />
      ※
      1回のお会計で1頒布物あたり1点のみ購入できます。複数点をご希望の場合は購入後に最初からやり直すか、有人頒布をご利用ください。
    </p>
    <div class="list">
      <div class="item" v-for="item in items" :key="item.id">
        <div class="checkbox">
          <input
            type="checkbox"
            :value="item"
            v-model="cart"
            @change="onChange"
            :disabled="isFinish"
          />
        </div>
        <div class="image">
          <img :src="getImageUrl(item.id)" />
        </div>
        <div class="name">
          {{ item.name }}
        </div>
        <div class="price">￥ {{ item.price }}</div>
      </div>
    </div>
    <div class="total">
      <div class="left">合計</div>
      <div class="right">￥ {{ total }}</div>
    </div>
    <h1>Step. 2</h1>
    <p>
      上記の合計と同じ額の現金を手元に用意してから「購入する」ボタンを押してください。<br />
      ※ 外貨及びキャッシュレス決済は有人頒布のみご利用いただけます。<br />
      ※ お釣りは出ません。確認作業に影響が出るため、合計より多い額も入れないでください。
    </p>
    <div v-if="isMain">
      <div class="submit" @click="onSubmit">購入する</div>
    </div>
    <div v-if="isFinish" class="message">
      <p>
        お買い上げありがとうございます!<br />
        購入処理が完了しました。貯金箱へ上記の合計金額を入れた後に、チェックを入れた頒布物をお持ちください。<br />
        またのお越しをお待ちしております。
      </p>
      <p>※ このタブは閉じてください。</p>
    </div>
  </div>
  <div v-if="isCover" class="cover"></div>
</template>

<style lang="scss" scoped>
.main {
  padding: 10px 20px;

  h1 {
    margin: 10px 0;
    font-size: 18px;
    font-weight: normal;
  }

  p {
    margin: 10px 0;
  }

  .list {
    margin: 20px 0;

    .item {
      overflow: hidden;

      .checkbox,
      .image,
      .name,
      .price {
        float: left;
      }

      .checkbox {
        width: 5%;
      }

      .image {
        width: 20%;

        img {
          width: 100%;
        }
      }

      .name {
        width: 60%;
      }

      .price {
        width: 15%;
        text-align: right;
      }
    }
  }

  .total {
    overflow: hidden;

    .left {
      padding-top: 8px;
      float: left;
      color: #757575;
      font-size: 16px;
    }

    .right {
      float: right;
      font-size: 24px;
    }
  }

  .submit {
    height: 40px;
    margin: 30px 0;
    padding-top: 6px;
    color: #fff;
    background-color: #e91e63;
    text-align: center;
    font-size: 16px;
    border-radius: 20px;

    &:hover {
      background-color: #d81b60;
      cursor: pointer;
    }
  }

  .message {
    margin: 30px 0;
    padding: 2px 16px;
    color: #1b5e20;
    background-color: #c8e6c9;
    border-radius: 6px;
  }
}

.cover {
  width: 100%;
  height: 100%;
  position: fixed;
  left: 0;
  top: 0;
  background-color: #000;
  opacity: 0.5;
  z-index: 100;
}
</style>
