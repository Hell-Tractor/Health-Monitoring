<template>
  <div>
    <el-row :gutter="20">
      <el-col :span="7" style="margin-left: 40px;">
        <stats-card>
          <div class="icon-big text-center" :class="`icon-danger`" slot="header">
            <i class="ti-heart"></i>
          </div>
          <div class="numbers" slot="content">
            <p>心率</p>{{this.heartRate}}
          </div>
          <div class="stats" slot="footer">
            <i class="ti-reload"></i>Updated now
          </div>
        </stats-card>
      </el-col>
      <el-col :span="7" style="margin-left: 40px;">
        <stats-card>
          <div class="icon-big text-center" :class="`icon-success`" slot="header">
            <i class="ti-image"></i>
          </div>
          <div class="numbers" slot="content">
            <p>血压</p>{{this.bloodPressureLow}}/{{this.bloodPressureHigh}}
          </div>
          <div class="stats" slot="footer">
            <i class="ti-reload"></i>Updated now
          </div>
        </stats-card>
      </el-col>
      <el-col :span="7" style="margin-left: 40px;">
        <stats-card>
          <div class="icon-big text-center" :class="`icon-info`" slot="header">
            <i class="ti-pulse"></i>
          </div>
          <div class="numbers" slot="content">
            <p>血氧</p>{{this.bloodOxygen}}
          </div>
          <div class="stats" slot="footer">
            <i class="ti-reload"></i>Updated now
          </div>
        </stats-card>
      </el-col>
    </el-row>
    <el-row :gutter="20" style="margin-top: 10px;">
      <el-col :span="11" style="margin-left: 40px;">
        <div class="card">
          <div class="card-body">
            <div id="echarts" style="width: 500px; height: 305px; margin-top: 25px;"></div>
          </div>
        </div>
      </el-col>
      <el-col style="width: 578px; margin-left: 10px">
        <div class="card">
          <div class="card-body">
            <div id="distance_chart" style="width: 500px; height: 320px; margin-top: 10px;"></div>
          </div>
        </div>
      </el-col>
    </el-row>
  </div>
  
</template>

<script>
import StatsCard from "@/components/Cards/StatsCard";
import dataApi from '@/api/data';
import * as echarts from 'echarts';

export default {
  components: {
    StatsCard,
  },
  data() {
    return {
      heartRate:'',
      bloodPressureLow:'',
      bloodPressureHigh:'',
      bloodOxygen:'',
      chart: null,
      distanceChart: null,
    }
  },
  async created() {
    this.heartRate = await dataApi.getHeartRate().then(re => {
      return re.data[0].value
    })
    this.bloodPressureLow =await dataApi.getBloodPressureLow().then(re => {
      return re.data[0].value
    })
    this.bloodPressureHigh =await dataApi.getBloodPressureHigh().then(re => {
      return re.data[0].value
    })
    this.bloodOxygen =await dataApi.getBloodOxygen().then(re => {
      return re.data[0].value
    })
  },
  methods: {
    async update() {
      this.heartRate = await dataApi.getHeartRate().then(re => {
        return re.data[0].value
      })
      this.bloodPressureLow =await dataApi.getBloodPressureLow().then(re => {
        return re.data[0].value
      })
      this.bloodPressureHigh =await dataApi.getBloodPressureHigh().then(re => {
        return re.data[0].value
      })
      this.bloodOxygen =await dataApi.getBloodOxygen().then(re => {
        return re.data[0].value
      })
    },
    init1() {
      // 初始化
      this.chart = echarts.init(document.getElementById("echarts"));
      // 配置数据
      let option = {
        tooltip: {
          trigger: 'axis',
          axisPointer: {
            type: 'cross',
            crossStyle: {
              color: '#999'
            }
          }
        },
      toolbox: {
        feature: {
          //dataView: { show: true, readOnly: false },
          magicType: { show: true, type: ['line', 'bar'] },
          restore: { show: true },
          //saveAsImage: { show: true }
        }
      },
      legend: {
        data: ['已爬楼层', '步数']
      },
      xAxis: [
        {
          type: 'category',
          data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'],
          axisPointer: {
            type: 'shadow'
          }
        }
      ],
      yAxis: [
        {
          type: 'value',
          name: '已爬楼层',
          min: 0,
          max: 250,
          interval: 50,
          axisLabel: {
            formatter: '{value}'
          }
        },
        {
          type: 'value',
          name: '步数',
          min: 0,
          max: 25,
          interval: 5,
          axisLabel: {
            formatter: '{value}'
          }
        }
      ],
      series: [
        {
          name: '已爬楼层',
          type: 'bar',
          tooltip: {
            valueFormatter: function (value) {
              return value;
            }
          },
          data: [
            2.0, 4.9, 7.0, 23.2, 25.6, 76.7, 135.6, 162.2, 32.6, 20.0, 6.4, 3.3
          ]
        },
        {
          name: '步数',
          type: 'line',
          yAxisIndex: 1,
          tooltip: {
            valueFormatter: function (value) {
              return value;
            }
          },
          data: [2.0, 2.2, 3.3, 4.5, 6.3, 10.2, 20.3, 23.4, 23.0, 16.5, 12.0, 6.2]
        }
      ]
      };
      // 传入数据
      this.chart.setOption(option);
    },
    init2() {
      this.distanceChart = echarts.init(document.getElementById("distance_chart"));
      let option = {
        tooltip: {
          trigger: 'axis',
          axisPointer: {
            // Use axis to trigger tooltip
            type: 'shadow' // 'shadow' as default; can also be 'line' or 'shadow'
          }
        },
        legend: {},
        grid: {
          left: '3%',
          right: '4%',
          bottom: '3%',
          containLabel: true
        },
        xAxis: {
          type: 'value'
        },
        yAxis: {
          type: 'category',
          data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
        },
        series: [
          {
            name: '步行距离',
            type: 'bar',
            stack: 'total',
            label: {
              show: true
            },
            emphasis: {
              focus: 'series'
            },
            data: [320, 302, 301, 334, 390, 330, 320]
          },
          {
            name: '跑步距离',
            type: 'bar',
            stack: 'total',
            label: {
              show: true
            },
            emphasis: {
              focus: 'series'
            },
            data: [120, 132, 101, 134, 90, 230, 210]
          }
        ]
      };
      this.distanceChart.setOption(option);
    }
  },
  mounted() {
    this.init1(),
    this.init2(),
    this.update()
  }
}
</script>

<style lang="scss" scoped>
</style>
