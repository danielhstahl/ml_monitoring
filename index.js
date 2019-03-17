const {
    getDriftForEachVariable,
    createInitialInputMetrics,
    updateInputMetrics,
    getKeys,
    generateConfusionMatrix
}=require('./js/metrics')
const fastify = require('fastify')({
    logger: true
})
const HISTORY_TO_KEEP=100
let inputMetrics //because global variables jst lying around are a good idea :|

const makeArrayIfNotArray=objOrArray=>Array.isArray(objOrArray)?objOrArray:[objOrArray]
// Declare a route
fastify.get('/', (request, reply) => {
    reply.send({ hello: 'world' })
})

fastify.post('/input', (request, reply)=>{
    const {inputValues, originalValues}=request.body
    const values=makeArrayIfNotArray(inputValues)
    if(inputMetrics){
        const keys=getKeys(values)
        inputMetrics=updateInputMetrics(keys, inputMetrics, values, HISTORY_TO_KEEP)
    }
    else {
        inputMetrics=createInitialInputMetrics(values, HISTORY_TO_KEEP)
    }
    reply.send({success:true})
})

fastify.get('/input', (request, reply)=>{
    reply.send({inputMetrics})
})


// Run the server!
fastify.listen(3000, (err, address) => {
    if (err) throw err
    fastify.log.info(`server listening on ${address}`)
})