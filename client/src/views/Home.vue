<template>
  <div>
    <el-row :gutter="20">
      <el-col :span="7" :offset="3">
        <stats-card>
          <div class="icon-big text-center" :class="`icon-info`" slot="header">
            <i class="ti-home"></i>
          </div>
          <div class="numbers" slot="content">
            <p>门店数量</p>{{this.storeNum}}
          </div>
          <div class="stats" slot="footer">
            <i class="ti-reload"></i>Updated now
          </div>
        </stats-card>
      </el-col>
      <el-col :span="7" :offset="2">
        <stats-card>
          <div class="icon-big text-center" :class="`icon-success`" slot="header">
            <i class="ti-user"></i>
          </div>
          <div class="numbers" slot="content">
            <p>员工数量</p>{{this.staffNum}}
          </div>
          <div class="stats" slot="footer">
            <i class="ti-reload"></i>Updated now
          </div>
        </stats-card>
      </el-col>
    </el-row>
  <!--
  <div id="customizedCalendar">
    <el-calendar
        :first-day-of-week=7
        v-model="value">
      <template
          slot="dateCell"
          slot-scope="{date, data}">
        <div slot="reference" class="div-Calendar" @click="clickCalendar(data)" style="position: relative;z-index: 10">
          <p>{{ data.day.split('-').slice(2).join('-') }}</p>
        </div>
        <div v-if="data.isSelected" id="selectP"></div>
      </template>
    </el-calendar>
    <div id="button">
      <el-button @click="skip('preYear')" type="primary" round size="mini"><i class="el-icon-arrow-left"></i>年
      </el-button>
      <el-button @click="skip('preMonth')" type="warning" round size="mini"><i class="el-icon-arrow-left"></i>月
      </el-button>
      <el-button @click="skip('preDay')" type="success" round size="mini"><i class="el-icon-arrow-left"></i>日
      </el-button>
      <el-button @click="skip('today')" type="info" round size="mini">今天</el-button>
      <el-button @click="skip('postDay')" type="success" round size="mini">日<i class="el-icon-arrow-right"></i>
      </el-button>
      <el-button @click="skip('postMonth')" type="warning" round size="mini">月<i class="el-icon-arrow-right"></i>
      </el-button>
      <el-button @click="skip('postYear')" type="primary" round size="mini">年<i class="el-icon-arrow-right"></i>
      </el-button>
    </div>
  </div>
-->
    <el-row>
      <el-col :offset="2">
    <div class="col-10">
    <el-calendar/>
    </div>
      </el-col>
    </el-row>
  </div>
</template>

<script>
import moment from 'moment'
import PubSub from 'pubsub-js'
import StatsCard from "@/components/Cards/StatsCard";
import storeApi from '@/api/store'
import staffApi from '@/api/staff'

export default {
  components: {
    StatsCard,
  },
  name: "Calendar",
  data() {
    return {
      value: new Date(),
      locale: 'zh',
      labels: {
        zh: {
          weekdayHeaderFormat: 'narrow',
          labelPrevDecade: '过去十年',
          labelPrevYear: '上一年',
          labelPrevMonth: '上个月',
          labelCurrentMonth: '当前月份',
          labelNextMonth: '下个月',
          labelNextYear: '明年',
          labelNextDecade: '下一个十年',
          labelToday: '今天',
          labelSelected: '选定日期',
          labelNoDateSelected: ' ',
          labelCalendar: '日历',
          labelNav: '日历导航',
          labelHelp: ' '
        }
      },
      storeNum:'',
      staffNum:'',
    }
  },
  async created() {
    this.storeNum = await storeApi.get().then(re => {
      return re.data.list.length
    })
    this.staffNum =await staffApi.get().then(re => {
      return re.data.list.length
    })
  },
  computed: {
    selectDate() {
      return moment(this.value).format("YYYY-MM-DD");
    }
  },
  methods: {
    skip(flag) {
      if (flag === 'preYear') this.value = new Date(this.value.setFullYear(this.value.getFullYear() - 1));
      else if (flag === 'preMonth') this.value = new Date(this.value.setMonth(this.value.getMonth() - 1));
      else if (flag === 'preDay') this.value = new Date(this.value.setDate(this.value.getDate() - 1));
      else if (flag === 'today') this.value = new Date();
      else if (flag === 'postDay') this.value = new Date(this.value.setDate(this.value.getDate() + 1));
      else if (flag === 'postMonth') this.value = new Date(this.value.setMonth(this.value.getMonth() + 1));
      else if (flag === 'postYear') this.value = new Date(this.value.setFullYear(this.value.getFullYear() + 1));
    },
    clickCalendar(data) {
      console.log(data)
      PubSub.publish("uploadWarningResultWarningTime", data.day);
    },
    async update() {
      this.storeNum = await storeApi.get().then(re => {
        return re.data.list.length
      })
      this.staffNum = await staffApi.get().then(re => {
        return re.data.list.length
      })
    }
  },
  mounted() {
    this.update()
  }
}
</script>

<style lang="scss" scoped>
#customizedCalendar {
  margin-left: 10.5%;
  width: 80%;
  height: 100%;
  border-radius: 10%;
  #button {
    margin-top: 10px;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  #selectP { //选中后圆圈颜色
    width: 30px;
    height: 30px;
    background-color: #ACC8CE;
    position: absolute;
    border-radius: 50%;
    opacity: 0.6;
  }

  ::v-deep .el-calendar__header {
    // 修改头部背景颜色
    background-color: #C5D1D4;
    padding: 3px 5px;
    border: none;
    display: flex;
    justify-content: center;

    .el-calendar__button-group {
      // 隐藏原生按钮
      display: none;
    }

    .el-calendar__title {
      // 修改头部标题的字体颜色
      color: white !important;
      font-size: 18px;
      font-weight: bolder;
    }
  }

  ::v-deep .el-calendar__body {
    // 修改主题部分
    padding: 0;
  }

  ::v-deep .el-calendar-table {
    thead {
      th {
        // 修改头部星期部分
        padding: 0;
        background-color: #C5D1D4;
        color: white;
        text-align: center;
      }
    }

    .is-selected {
      .el-calendar-day {
        p {
          // 选中日期颜色
          color: rgb(0, 0, 0);
        }
      }
    }

    .el-calendar-day {
      // 每天的小块样式设置
      padding: 0;
      height: 50px;
      display: flex;
      justify-content: center;
      align-items: center;
      text-align: center; //文本居中
      p {
        // 所有日期颜色
        color: black;
        z-index: 1;
        user-select: none;
        display: flex;
        margin-top: 80%;
      }
    }
  }

  ::v-deep .el-calendar-table__row {
    .prev, .next {
      // 修改非本月
      .el-calendar-day {
        p {
          color: #f0d9d5;
        }
      }
    }

    td {
      // 修改每一个日期td标签
      &:first-child, &:last-child {
        background-color: #f5f5f5;
      }
    }
  }

  button {
    padding: 3px 10px;
  }
  .el-button--success{ //日按钮
    background: #B4D9E1;
    border-color: #B4D9E1;
  }
  .el-button--warning{ //月按钮
    background: #98C6D0;
    border-color: #98C6D0;
  }
  .el-button--primary{ //年按钮
    background: #80B1BA;
    border-color: #80B1BA;
  }
  .el-button--info{ //今天按钮
    background: #C7E8EF;
    border-color: #C7E8EF;
  }
}

</style>
