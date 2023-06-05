<template>
  <div>
    <el-row :gutter="20"  style="margin-top: 20px;">
      <el-col :span="7" style="margin-left: 20px;">
        <div class="card">
          <div class="card-body">
            <el-descriptions title="健康小贴士" column=1 style="margin-left: 10px;">
              <el-descriptions-item label="wjx言" colon=false>多读书 多看报 少吃零食 多睡觉</el-descriptions-item>
              <el-descriptions-item label="cym言" colon=false>健康生活 是幸福的开始</el-descriptions-item>
              <el-descriptions-item label="老舍言" colon=false>健壮比美更重要</el-descriptions-item>
              <el-descriptions-item label="爱默生言" colon=false>健康是人生第一财富</el-descriptions-item>
            </el-descriptions>
          </div>
        </div>
      </el-col>
      <el-col :span="16" style="margin-left: 10px;">
        <div class="card">
          <div class="card-body">
            <div id="difference_chart" style="width: 770px; height: 175px; margin-top: 5px;"></div>
          </div>
        </div>
      </el-col>
    </el-row>
    <el-row :gutter="25"  style="margin-top: 20px;">
      <el-col :span="23" style="margin-left: 20px;">
        <div class="card">
          <div class="card-body">
            <div id="year_chart" style="width: 1100px; height: 250px; margin-top: -20px; margin-left: 15px"></div>
          </div>
        </div>
      </el-col>
    </el-row>
  </div>
</template>

<script>
import StatsCard from "@/components/Cards/StatsCard";
import staticsApi from '@/api/statics';
import * as echarts from 'echarts';

export default {
  components: {
    StatsCard,
  },
  data() {
    return {
      differentChart: null,
      yearChart: null,
      walkingDistance: [],
      runningDistance: [],
      stepData: [],
    }
  },
  async created() {
    await staticsApi.getWalkDistanceForTwoWeeks().then(re => {
      for(var i=0;i<re.data.length;i++) {
        this.walkingDistance.push(re.data[i].value)
      }
      console.log(this.walkingDistance)
    })
    await staticsApi.getRunDistanceForTwoWeeks().then(re => {
      for(var i=0;i<re.data.length;i++) {
        this.runningDistance.push(re.data[i].value)
      }
    })
    this.different_chart_init()
    this.getData('2023')
    this.year_chart_init()
  },
  methods: {
    async update() {
      await staticsApi.getWalkDistanceForTwoWeeks().then(re => {
        for(var i=0;i<re.data.length;i++) {
          this.walkingDistance[i]=(re.data[i].value)
        }
      })
      await staticsApi.getRunDistanceForTwoWeeks().then(re => {
        for(var i=0;i<re.data.length;i++) {
          this.runningDistance[i]=(re.data[i].value)
        }
      })
      this.different_chart_init()
      this.getData('2023')
      this.year_chart_init()
    },
    getData(year) {
        const date = +echarts.time.parse(year + '-01-01');
        const end = +echarts.time.parse(+year + 1 + '-01-01');
        const dayTime = 3600 * 24 * 1000;
        const data = [];
        for (let time = date; time < end; time += dayTime) {
          data.push([
            echarts.time.format(time, '{yyyy}-{MM}-{dd}', false),
            Math.floor(-1)
          ]);
        }
        staticsApi.getDailyStep().then(re => {
          for (var i=0;i<data.length;i++){
            for(var j=0;j<re.data.length;j++) {
              if(re.data[j].time.includes(data[i][0])){
                data[i][1]=re.data[j].value
                console.log(data[i][0])
              }
            }
          }
          this.stepData=data
          console.log(this.stepData)
        })
        return data;
      },
    different_chart_init() {
      // 初始化
      this.differentChart = echarts.init(document.getElementById("difference_chart"));
      // 配置数据
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
          data: ['本周', '上周']
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
            data: this.walkingDistance
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
            data: this.runningDistance
          }
        ]
      };
      // 传入数据
      this.differentChart.setOption(option);
    },
    year_chart_init() {
      // 初始化
      this.yearChart = echarts.init(document.getElementById("year_chart"));
      // 配置数据
      
      let option = {
        title: {
          top: 30,
          left: 'center',
          text: '每日步数记录'
        },
        tooltip: {},
        visualMap: {
          min: 0,
          max: 20000,
          type: 'piecewise',
          orient: 'horizontal',
          left: 'center',
          top: 65
        },
        calendar: {
          top: 120,
          left: 30,
          right: 30,
          cellSize: ['auto', 13],
          range: '2023',
          itemStyle: {
            borderWidth: 0.5
          },
          yearLabel: { show: false }
        },
        series: {
          type: 'heatmap',
          coordinateSystem: 'calendar',
          data: this.stepData
        }
      };
      this.yearChart.setOption(option)
    }
  },
  mounted() {
    this.different_chart_init(),
    this.getData('2023'),
    this.year_chart_init()
  }
}
</script>

<style lang="scss" scoped>
</style>