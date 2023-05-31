<template>
  <div>
    <el-row :gutter="20">
      <el-col :span="7" :offset="1">
        <stats-card>
          <div class="icon-big text-center" :class="`icon-danger`" slot="header">
            <i class="ti-heart"></i>
          </div>
          <div class="numbers" slot="content">
            <p>心率</p>{{this.storeNum}}
          </div>
          <div class="stats" slot="footer">
            <i class="ti-reload"></i>Updated now
          </div>
        </stats-card>
      </el-col>
      <el-col :span="7" :offset="1">
        <stats-card>
          <div class="icon-big text-center" :class="`icon-success`" slot="header">
            <i class="ti-image"></i>
          </div>
          <div class="numbers" slot="content">
            <p>血压</p>{{this.staffNum}}
          </div>
          <div class="stats" slot="footer">
            <i class="ti-reload"></i>Updated now
          </div>
        </stats-card>
      </el-col>
      <el-col :span="7" :offset="1">
        <stats-card>
          <div class="icon-big text-center" :class="`icon-info`" slot="header">
            <i class="ti-pulse"></i>
          </div>
          <div class="numbers" slot="content">
            <p>血氧</p>{{this.storeNum}}
          </div>
          <div class="stats" slot="footer">
            <i class="ti-reload"></i>Updated now
          </div>
        </stats-card>
      </el-col>
    </el-row>
    <el-row :gutter="20">
      <el-col :span="12" :offset="1">
        <stats-card>
          <div id="echarts" style="width: 300px; height: 300px;" slot="content"></div>
        </stats-card>
      </el-col>
    </el-row>
  </div>
  
</template>

<script>
import StatsCard from "@/components/Cards/StatsCard";
import storeApi from '@/api/store'
import staffApi from '@/api/staff'

let Echarts = require('echarts/lib/echarts'); //基础实例 注意不要使用import
require('echarts/lib/chart/bar'); //按需引入 bar = 柱状图

export default {
  components: {
    StatsCard,
  },
  data() {
    return {
      storeNum:'',
      staffNum:'',
      chart: null
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
  methods: {
    async update() {
      this.storeNum = await storeApi.get().then(re => {
        return re.data.list.length
      })
      this.staffNum = await staffApi.get().then(re => {
        return re.data.list.length
      })
    },
    init() {
      //2.初始化
      this.chart = Echarts.init(document.getElementById("echarts"));
      //3.配置数据
      let option = {
        xAxis: { type: 'category', data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'] }, //X轴
        yAxis: { type: 'value' }, //Y轴
        series: [{ data: [120, 200, 150, 80, 70, 110, 130], type: 'bar' }] //配置项
      };
      // 4.传入数据
      this.chart.setOption(option);
    }
  },
  mounted() {
    this.init(),
    this.update()
  }
}
</script>

<style lang="scss" scoped>

</style>
