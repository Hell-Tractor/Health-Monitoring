<template>
  <div>
    <el-row :gutter="20">
      <el-col :span="12" :offset="1">
        <div class="card">
          <div class="card-body">
            <div id="echarts" style="width: 550px; height: 300px; margin-top: 30px;"></div>
          </div>
        </div>
      </el-col>
      <el-col :span="11" :offset="0">
        <div class="card">
          <div class="card-body">
            <div id="distance_chart" style="width: 450px; height: 315px; margin-top: 15px;"></div>
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
      bloodPressure:'',
      bloodOxygen:'',
      chart: null,
      distanceChart: null,
    }
  },
  async created() {
    this.heartRate = await dataApi.getHeartRate().then(re => {
      return re.data[0].value
    })
    this.bloodPressure =await dataApi.getBloodPressure().then(re => {
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
      this.bloodPressure = await dataApi.getBloodPressure().then(re => {
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
        angleAxis: {
          type: 'category',
          data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
        },
        radiusAxis: {},
        polar: {},
        series: [
          {
            type: 'bar',
            data: [1, 2, 3, 4, 3, 5, 1],
            coordinateSystem: 'polar',
            name: 'A',
            stack: 'a',
            emphasis: {
              focus: 'series'
            }
          },
          {
            type: 'bar',
            data: [2, 4, 6, 1, 3, 2, 1],
            coordinateSystem: 'polar',
            name: 'B',
            stack: 'a',
            emphasis: {
              focus: 'series'
            }
          },
          {
            type: 'bar',
            data: [1, 2, 3, 4, 1, 2, 5],
            coordinateSystem: 'polar',
            name: 'C',
            stack: 'a',
            emphasis: {
              focus: 'series'
            }
          }
        ],
        legend: {
          show: true,
          data: ['A', 'B', 'C']
        }
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